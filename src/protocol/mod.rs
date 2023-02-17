use std::{collections::{BTreeMap, HashMap}};
use serde::{Deserialize, Serialize};

use nom_helper::hexadecimal_u16_value;

pub mod field;
pub mod fieldtype;
pub mod fieldapp;
pub mod nom_helper;
pub mod command;

pub use command::Command;

static PROTOCOL: &[u8] = include_bytes!("../../protocol/protocol-en.json");

#[derive(Serialize, Deserialize, Debug)]
pub struct Protocol {
    pub id: String,
    pub name: String,
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub version: i16,
    #[serde(rename="queryCommands")]
    #[serde(default)]
    pub query_commands: Vec<String>,
    pub commands: Vec<Command>,
}


impl Protocol {
    pub fn new() -> HashMap<String, Protocol> {
        let protocol_str = String::from_utf8_lossy(PROTOCOL);
        let protocol: serde_json::Result<HashMap<String, Protocol>> = serde_json::from_str(&protocol_str);
        protocol.unwrap()
    }

    pub fn listing(&self, current_version: i16) {
        let cmds = self.get_commands(current_version);

        for c in cmds {
            if c.1.op == 1 && c.1.cmd != "BB" {
                println!("sermatec-ess get --el {:02x} : {}", c.0, c.1.comment);
            } else {
                println!("sermatec-ess get --el {:02x} : {} (*)", c.0, c.1.comment);
            }
        }
        println!("(*) DO NOT USE!");
    }

    pub fn get_command(&self, version: i16, command: &str) -> Option<&Command> {
        for e in &self.versions {
            if e.version == version {
                for cmd in &e.commands {
                    if cmd.cmd == *command {
                        return Some(cmd);
                    }
                }
            }
        }
        return None;
    }
    
    pub fn get_commands(&self, pcu_version: i16) -> BTreeMap<u16, &Command> {
        let mut commands: BTreeMap<u16, &Command> = BTreeMap::new();

        for e in &self.versions {
            // we overwrite old cmd versions
            if e.version <= pcu_version {
                for cmd in &e.commands {
                    let (_input, c) = hexadecimal_u16_value(&cmd.cmd).unwrap();
                    commands.insert(c, cmd);
                }
            }
        }
        return commands;
    }

}
