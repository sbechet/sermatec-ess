use std::collections::BTreeMap;
use std::io::prelude::*;
use std::time::Duration;
use std::net::TcpStream;
use std::thread;

use rumqttc::{MqttOptions, Client, QoS};

use crate::protocol::Command;



pub struct Daemon {
    host: String,
    port: u16,
}


impl Daemon {
    pub fn new(host: &str, port: u16) -> Daemon {
        Daemon {
            host: host.to_string(),
            port: port,
        }
    }

    /*
        what can we send to home assistant?
        0a: battery soc
        0b: PVI power, PV2 power, PV total power
        1f: sysFault2: [28, 0] ?

     */
    pub fn run(&self, stream: &mut TcpStream, cmds: &BTreeMap<u16, &Command>) -> Result<(), String> {
        let mut mqttoptions = MqttOptions::new("rumqtt-sync", &self.host, self.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        client.subscribe("sermatec-ess", QoS::AtMostOnce).unwrap();


        let cmd_value: u16 = 0x000A; // Battery info
        let cmd = cmds[&cmd_value];
        let battery_packet = cmd.build_packet().unwrap();

        println!("MQTT: Sending to sermatec-ess/# ...");

        for notification in connection.iter() {
            println!("MQTT: Notification = {:?}", notification);


            stream.write(&battery_packet).unwrap();
            let elements = cmd.parse_answer(stream);
            match &elements {
                Ok(elts) => {
                    for fa in elts {
                        if fa.f.name == "battery soc" {
                            let t = format!("{:?}", fa.v);
                            let t = t.as_bytes();
                            client.publish("sermatec-ess/battery/soc", QoS::AtLeastOnce, false, t).unwrap();
                        }
                    }
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            }

            // minimal sleep then mqtt ping/pong
            thread::sleep(Duration::from_secs(5*60));

        }
        Ok(())
    }
}