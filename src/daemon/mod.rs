use std::collections::BTreeMap;
use std::io::prelude::*;
use std::time::Duration;
use std::net::{ SocketAddr, TcpStream };
use std::thread;

use rumqttc::{MqttOptions, Client, QoS};
use chrono::{DateTime, Utc};

use crate::protocol::Command;
use crate::protocol::fieldapp::FieldApp;
use crate::Hardware;


pub struct Daemon<'a> {
    hardware: Hardware,
    sermatec_socket: SocketAddr,
    stream: TcpStream,
    host: String,
    port: u16,
    cmds: BTreeMap<u16, &'a Command>,
    wait: Duration,
    wait_counter: usize,
    wait_current: usize,
}

impl<'a> Daemon<'a> {
    pub fn new(hardware: Hardware, sermatec_socket: SocketAddr, stream: TcpStream, host: &str, port: u16, cmds: BTreeMap<u16, &'a Command>, wait: u16) -> Daemon<'a> {
        Daemon {
            hardware: hardware,
            sermatec_socket: sermatec_socket,
            stream: stream,
            host: host.to_string(),
            port: port,
            cmds: cmds,
            wait: Duration::from_secs(wait.into()),
            wait_counter: 1*3600 / wait as usize, // Send every hour
            wait_current: 1*3600 / wait as usize,
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
        let node_id = &self.hardware.product_sn;
        format!("{}/{}/{}", discovery_prefix, component, node_id)
    }

    /// Return hass topic from field
    fn hass_get_topic(&self, fa: &FieldApp) -> String {
        let root = self.hass_get_root_topic();
        let object_id = fa.c.comment.replace(" ", "_").to_lowercase();
        let object_id = object_id.replace("/", "_").to_lowercase();
        format!("{}/{}", root, object_id)
    }

    fn hass_name_cleanup(&self, fa: &FieldApp) -> String {
        let prefix: &str = if let "0A" = fa.c.cmd.as_str() {
            "battery_"
        } else {
            "sermatec_"
        };
        let mut name = String::from(prefix);
        name.push_str(&fa.f.name);

        let name = name.replace(" ", "_");
        let name = name.replace("-", "_");
        let name = name.replace("(", "_").replace(")", "_").replace(":", "_");
        let name = name.replace("/", "_").to_lowercase();

        name
    }

    fn hass_name_uniq(&self, fa: &FieldApp) -> String {
        let mut name = String::from(&self.hardware.product_sn).to_lowercase();
        name.push_str("_");
        name.push_str(&self.hass_name_cleanup(fa));
        name
    }

    /// Return hass Sensor MQTT autodiscovery (topic config, payload) for field
    fn hass_config(&self, fa: &FieldApp) -> Option< (String, String) > {
        let name = self.hass_name_cleanup(fa);
        // let state_class = "measurement";
        let device_class_option = self.hass_get_device_class(fa);

        let topic_root = self.hass_get_topic(fa);
        let topic_name = format!("{}_{}", topic_root, name);
        let topic_config = format!("{}/config", topic_name);
        let topic_state = format!("{}/state", topic_root);

        let manufacturer: &str = match fa.c.cmd.as_str() {
            "0A" => &self.hardware.battery_name,
            _ => "Sermatec",
        };

        let model = match manufacturer {
            "Sermatec" => &self.hardware.model_code,
            _ => "",
        };

        let product_sn = match manufacturer {
            "Sermatec" => &self.hardware.product_sn,
            _ => "",
        };

        let unique_id = self.hass_name_uniq(fa);

        let payload_config_option = match device_class_option {
            Some(device_class) => {
                let pc = format!(r###"{{
                    "device_class": "{}",
                    "name": "{}",
                    "state_topic": "{}",
                    "unique_id": "{}",
                    "unit_of_measurement": "{}",
                    "value_template": "{{{{ value_json.{} }}}}",
                    "device": {{ "manufacturer": "{}", "model": "{}", "identifiers": "{}" }}
                }}"###, device_class, fa.f.name, topic_state, unique_id, fa.f.unit_type, name, manufacturer, model, product_sn);
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
    fn config(&mut self, cmds_value: &[u16]) -> Vec<(String, String)> {
        let mut configs: Vec<(String, String)> = vec![];

        for cmd_value in cmds_value {
            let cmd = self.cmds[cmd_value];
            let packet = cmd.build_packet().unwrap();
            self.write(&packet);
            let elements = cmd.parse_answer(&mut self.stream);
            match &elements {
                Ok(elts) => {
                    for fa in elts {
                        match self.hass_config(fa) {
                            Some( (k,v) ) => configs.push( (k, v) ),
                            None => (),
                        }
                    }
                },
                Err(e) => {
                    println!("Error, config({:02X}): {}", cmd_value, e);
                },
            }
        }

        return configs;
    }

    // (name, value)
    fn hass_update(&self, fa: &FieldApp) -> Option< (String, String) > {
        let device_class_option = self.hass_get_device_class(fa);
        let payload_kv_state_option = match device_class_option {
            Some(_) => {
                let name = self.hass_name_cleanup(fa);
                Some( (name, format!("{:?}", fa)) )
            },
            _ => None,
        };
        match payload_kv_state_option {
            Some(kv) => Some(kv),
            None => None
        }
    }

    fn write(&mut self, buf: &[u8]) {
        loop {
            match self.stream.write(buf) {
                Ok(_v) => {
                    return
                },
                Err(_e) => {
                    println!("Trying to reconnect");
                    thread::sleep(Duration::from_secs(2));
                    self.stream = TcpStream::connect(self.sermatec_socket).unwrap();
                },
            }
        }
    }

    fn update(&mut self, cmds_value: &[u16]) -> Vec<(String, String)> {
        let mut answers: Vec<(String, String)> = vec![];

        for cmd_value in cmds_value {
            let cmd = self.cmds[cmd_value];
            let packet = cmd.build_packet().unwrap();
            self.write(&packet);
            let elements = cmd.parse_answer(&mut self.stream);
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
                    answers.push( (topic_state, payload) );
                },
                Err(e) => {
                    println!("Error, update({:02X}): {}", cmd_value, e);
                    // no way
                    answers = vec![];
                    return answers;
                },
            }
        }

        return answers;
    }

    pub fn run(&mut self) -> ! {
        let cmds_value: [u16; 2] = [0x000A, 0x000B];

        let mut mqttoptions = MqttOptions::new("rumqtt-sync", &self.host, self.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        let topic = self.hass_get_root_topic();
        client.subscribe(topic, QoS::AtMostOnce).unwrap();

        thread::spawn(move || {
            for _notification in connection.iter() {
                // println!("MQTT: Notification = {:?}", _notification);
            }
        });

        let configs = self.config(&cmds_value);

        println!("MQTT: Sending config every {:?} seconds.", self.wait_counter * self.wait.as_secs() as usize);
        println!("MQTT: Sending states every {:?}.", self.wait);
        loop {
            let now: DateTime<Utc> = Utc::now();
            let nowfmt = now.format("%Y%m%d%H%M");

            if self.wait_current == self.wait_counter {
                println!("{} > MQTT: Sending Config to HomeAssistant", nowfmt);
                for (k, v) in &configs {
                    client.publish(k, QoS::AtLeastOnce, false, v.as_bytes()).unwrap();
                };
                self.wait_current = 0;
            } else {
                self.wait_current += 1;
            }


            let answers = self.update(&cmds_value);
            if answers.len() != 0 {
                for (k, v) in &answers {
                    println!("{} > {} = {}", nowfmt, k, v);
                    client.publish(k, QoS::AtLeastOnce, false, v.as_bytes()).unwrap();
                }
            } else {
                println!("WARN: No answer!");
            }

            thread::sleep(self.wait);

        }

        // no return

    }
}