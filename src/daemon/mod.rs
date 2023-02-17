use std::collections::BTreeMap;
use std::io::prelude::*;
use std::time::Duration;
use std::net::TcpStream;
use std::thread;

use rumqttc::{MqttOptions, Client, QoS};

use crate::protocol::Command;
use crate::protocol::fieldapp::FieldApp;


pub struct Daemon<'a> {
    product_sn: String,
    host: String,
    port: u16,
    cmds: BTreeMap<u16, &'a Command>,
    wait: Duration,
    configs: Vec<(String, String)>,
    answers: Vec<(String, String)>,
}

impl<'a> Daemon<'a> {
    pub fn new(product_sn: &str, host: &str, port: u16, cmds: BTreeMap<u16, &'a Command>, wait: u16) -> Daemon<'a> {
        Daemon {
            product_sn: product_sn.to_string(),
            host: host.to_string(),
            port: port,
            cmds: cmds,
            wait: Duration::from_secs(wait.into()),
            configs: vec![],
            answers: vec![],
        }
    }

    /// Return hass device_class from field
    fn hass_get_device_class(&self, fa: &FieldApp) -> Option<&str> {
        match fa.f.unit_type.as_str() {
            "%" => Some("battery"),
            "A" => Some("current"),
            "C" | "℃" => Some("temperature"),
            "HZ" => Some("frequency"),
            "V" => Some("voltage"),
            "VA" => Some("apparent_power"),
            "kVar" | "Var" => Some("reactive_power"),
            "kW" | "KW" | "W" => Some("power"),
            "€" | "d€" => Some("monetary"),
            _ => None,
        }
    }

    /// Return hass topic from field
    fn hass_get_root_topic(&self) -> String {
        let discovery_prefix = "homeassistant";
        let component = "sensor";
        let node_id = &self.product_sn;
        format!("{}/{}/{}", discovery_prefix, component, node_id)
    }

    /// Return hass topic from field
    fn hass_get_topic(&self, fa: &FieldApp) -> String {
        let root = self.hass_get_root_topic();
        let object_id = fa.c.comment.replace(" ", "_").to_lowercase();
        let object_id = object_id.replace("/", "_").to_lowercase();
        format!("{}/{}", root, object_id)
    }

    /// Return hass Sensor MQTT autodiscovery (topic config, payload) for field
    fn hass_config(&self, fa: &FieldApp) -> Option< (String, String) > {
        let name = fa.f.name.replace(" ", "_");
        let name = name.replace("-", "_");
        let name = name.replace("/", "_").to_lowercase();

        // let state_class = "measurement";
        let device_class_option = self.hass_get_device_class(fa);

        let topic_root = self.hass_get_topic(fa);
        let topic_name = format!("{}_{}", topic_root, name);
        let topic_config = format!("{}/config", topic_name);
        let topic_state = format!("{}/state", topic_root);

        let payload_config_option = match device_class_option {
            Some(device_class) => {
                let pc = format!(r###"{{
                    "device_class": "{}",
                    "name": "{}",
                    "state_topic": "{}",
                    "unit_of_measurement": "{}",
                    "value_template": "{{{{ value_json.{} }}}}"
                }}"###, device_class, fa.f.name, topic_state, fa.f.unit_type, name);
                Some(pc)
            },
            _ => None,
        };
        match payload_config_option {
            Some(payload_config) => Some( (topic_config, payload_config) ),
            None => None
        }
    }

    /// Call it only one time to update configs
    fn config(&mut self, stream: &mut TcpStream, cmds_value: &[u16]) {
        for cmd_value in cmds_value {
            let cmd = self.cmds[cmd_value];
            let packet = cmd.build_packet().unwrap();
            stream.write(&packet).unwrap();
            let elements = cmd.parse_answer(stream);
            match &elements {
                Ok(elts) => {
                    for fa in elts {
                        match self.hass_config(fa) {
                            Some( (k,v) ) => self.configs.push( (k, v) ),
                            None => (),
                        }
                    }
                },
                Err(e) => {
                    println!("Error, config({}): {}", cmd_value, e);
                },
            }
        }
    }

    // (name, value)
    fn hass_update(&mut self, fa: &FieldApp) -> Option< (String, String) > {
        let device_class_option = self.hass_get_device_class(fa);
        let payload_kv_state_option = match device_class_option {
            Some(_) => {
                let name = fa.f.name.replace(" ", "_");
                let name = name.replace("-", "_");
                let name = name.replace("/", "_").to_lowercase();
                Some( (name, format!("{:?}", fa)) )
            },
            _ => None,
        };
        match payload_kv_state_option {
            Some(kv) => Some(kv),
            None => None
        }
    }

    fn update(&mut self, stream: &mut TcpStream, cmds_value: &[u16]) {
        self.answers = vec![];

        for cmd_value in cmds_value {
            let cmd = self.cmds[cmd_value];
            let packet = cmd.build_packet().unwrap();
            stream.write(&packet).unwrap();
            let elements = cmd.parse_answer(stream);
            match &elements {
                Ok(elts) => {
                    // topic are the same for all Vector
                    let fa = &elts[0];
                    let topic = self.hass_get_topic(fa);
                    let topic_state = format!("{}/state", topic);

                    let mut payload = String::from("{");
                    for fa in elts {
                        match self.hass_update(fa) {
                            Some( (k, v) ) => {
                                let t = String::from(format!(r###""{}": {}, "###, k, v));
                                payload.push_str(&t);
                            },
                            None => (),
                        }
                    }
                    payload.remove(payload.len()-1);    // remove space
                    payload.remove(payload.len()-1);    // remove ,
                    let t = String::from(" }");
                    payload.push_str(&t);
                    self.answers.push( (topic_state, payload) );
                },
                Err(e) => {
                    println!("Error, config({}): {}", cmd_value, e);
                },
            }
        }
    }

    pub fn run(mut self, mut stream: TcpStream) -> ! {
        let cmds_value: [u16; 3] = [0x000A, 0x000B, 0x009D];

        let mut mqttoptions = MqttOptions::new("rumqtt-sync", &self.host, self.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        let topic = self.hass_get_root_topic();
        client.subscribe(topic, QoS::AtMostOnce).unwrap();


        self.config(&mut stream, &cmds_value);
        // TODO: Find how to copy self and stream in thread?
        self.update(&mut stream, &cmds_value);
        thread::spawn(move || {
            println!("MQTT: Sending Home Assistant MQTT Discovery data...");
            for (k, v) in &self.configs {
                println!("MQTT: Sending {} = {}", k, v);
                client.publish(k, QoS::AtLeastOnce, false, v.as_bytes()).unwrap();
            };
            println!("MQTT: Sending states every {:?} seconds...", self.wait);
            loop {
                for (k, v) in &self.answers {
                    println!("MQTT: Sending {} = {}", k, v);
                    client.publish(k, QoS::AtLeastOnce, false, v.as_bytes()).unwrap();
                }
                thread::sleep(self.wait);
            }
        });

        for notification in connection.iter() {
            println!("MQTT: Notification = {:?}", notification);
        }

        // no return
        loop {}
    }
}