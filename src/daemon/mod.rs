use std::collections::{ BTreeMap, HashMap };
use std::io::prelude::*;
use std::time::Duration;
use std::net::TcpStream;
use std::thread;

use rumqttc::{MqttOptions, Client, QoS};

use crate::protocol::Command;



pub struct Daemon<'a> {
    host: String,
    port: u16,
    cmds: &'a BTreeMap<u16, &'a Command>,
    answers: HashMap<String, Vec<u8>>,
}


impl<'a> Daemon<'a> {
    pub fn new(host: &str, port: u16, cmds: &'a BTreeMap<u16, &Command>) -> Daemon<'a> {
        Daemon {
            host: host.to_string(),
            port: port,
            cmds: cmds,
            answers: HashMap::new(),
        }
    }

    fn add_cmd(&mut self, stream: &mut TcpStream, cmds: &BTreeMap<u16, &Command>, cmd_value: u16) {
        let cmd = cmds[&cmd_value];
        let packet = cmd.build_packet().unwrap();
        stream.write(&packet).unwrap();
        let elements = cmd.parse_answer(stream);
        match &elements {
            Ok(elts) => {
                for fa in elts {

                    let cname = fa.c.comment.replace(" ", "_").to_lowercase();
                    // we want all
                    let name = fa.f.name.replace(" ", "_");
                    let name = name.replace("/", "_").to_lowercase();

                    let v = format!("{:?}", fa);
                    let v = v.as_bytes();

                    let k = format!("sermatec-ess/{}/{}", cname, name);

                    self.answers.insert(k,v.to_vec());

                }
            }
            Err(e) => {
                println!("Error, add_cmd({}): {}", cmd_value, e);
            }
        }
    }

    fn update(&mut self, stream: &mut TcpStream) {
        self.answers = HashMap::new();

        // Battery info
        self.add_cmd(stream, self.cmds, 0x000A);
        // Control cabinet information
        self.add_cmd(stream, self.cmds, 0x000B);
        // Equipment running status
        // self.add_cmd(stream, self.cmds, 0x000C);
        // bmsMeter connection status
        // self.add_cmd(stream, self.cmds, 0x000D);
        // BMS alarm information
        // self.add_cmd(stream, self.cmds, 0x001E);
        // System fault status
        // self.add_cmd(stream, self.cmds, 0x001F);
        // Total power data
        self.add_cmd(stream, self.cmds, 0x0099);
        // Total grid data
        self.add_cmd(stream, self.cmds, 0x009A);
        // Load power data
        self.add_cmd(stream, self.cmds, 0x009B);
        // Grid battery power data
        self.add_cmd(stream, self.cmds, 0x009C);
        // Set parameter information 2
        self.add_cmd(stream, self.cmds, 0x009D);


    }

    pub fn run(mut self, stream: &mut TcpStream) -> Result<(), String> {
        let mut mqttoptions = MqttOptions::new("rumqtt-sync", &self.host, self.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut connection) = Client::new(mqttoptions, 10);
        client.subscribe("sermatec-ess", QoS::AtMostOnce).unwrap();


        println!("MQTT: Sending to sermatec-ess/# Topic...");

        self.update(stream);
        thread::spawn(move || loop {
            for (k, v) in &self.answers {
                println!("MQTT: Sending {} = {:02X?}", k, v);
                client.publish(k, QoS::AtLeastOnce, false, v.clone()).unwrap();
            };
            thread::sleep(Duration::from_secs(5*60));
         });


        for notification in connection.iter() {
            println!("MQTT: Notification = {:?}", notification);
        }
        Ok(())
    }
}