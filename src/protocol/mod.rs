use std::{collections::{BTreeMap, HashMap}, io::Read};
use std::net::TcpStream;
use std::vec;
use nom::IResult;
use nom::bytes::complete::*;
use nom::number::complete::*;
use nom::{Err, error::ErrorKind, error::ParseError};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, PickFirst};
use hex;

pub mod field;
use field::Field;

pub mod fieldtype;
// use fieldtype::FieldType;

pub mod fieldapp;
use fieldapp::FieldApp;

pub mod nom_helper;
use nom_helper::hexadecimal_u16_value;

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

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    #[serde(rename="type")]
    pub cmd: String,
    pub comment: String,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub op: u8,
    pub fields: Vec<Field>
}

impl Protocol {
    pub fn new() -> HashMap<String, Protocol> {
        let protocol_str = String::from_utf8_lossy(PROTOCOL);
        let protocol: serde_json::Result<HashMap<String, Protocol>> = serde_json::from_str(&protocol_str);
        protocol.unwrap()
    }

    pub fn listing(&self, current_version: i16) {
        let cmds = self.get_commands(current_version);


        println!("> Not working: 0C A1 A2 <");
        for c in cmds {
            // TODO: To avoid risky commands, remove documentation for op2 and op3
            if c.1.op == 1 {
                println!("sermatec-ess get --el {:02x} : {}", c.0, c.1.comment);
            }
        }
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


impl Command {
    fn get_checksum(data: &[u8]) -> u8 {
        let mut checksum: u8 = 0x0f;
        
        for byte in data {
            checksum = (checksum & 0xff) ^ byte;
        }
        return checksum;
    }

    pub fn build_packet(&self) -> Option<Vec<u8>> {
        // Security: no op2 or op3 for now (read only)
        if self.op != 1 {
            println!("For now no op2 or op3!");
            return None;
        }

        let mut packet: Vec<u8> = vec![0xfe, 0x55, 0x64, 0x14]; // signature (0xfe, 0x55), req_app_addr (0x64), req_inv_addr (0x14)
        // **** command (u16)
        let mut c = hex::decode(&self.cmd).unwrap();
        // u16
        match c.len() {
            1 => { c.push(0); },
            2 => {},
            _ => panic!("We have a big problem with a protocol.json command description"),
        } 
        packet.append(&mut c);

        // **** payload len
        packet.push(0);

        // **** checksum
        packet.push(Self::get_checksum(&packet));

        //  **** footer
        packet.push(0xae);

        return Some(packet);
    }


    pub fn print_nice_answer(&self, answer: &Result<Vec<FieldApp>, String>) {
        match answer {
            Ok(fas) => {
                for fa in fas {
                    println!("{:?}", fa);
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    fn parse_sermatec_packet<'a>(&self, wanted_cmd: u16, stream: &'a [u8]) -> IResult<&'a [u8],&[u8]> {
        let (input, magic) = be_u16(stream)?;
        let (input, src) = be_u8(input)?;
        let (input, dst) = be_u8(input)?;
        let (input, cmd) = be_u16(input)?;
        let (input, payload_size) = be_u8(input)?;
        let (input, payload) = take(payload_size)(input)?;
        let (input, checksum) = be_u8(input)?;
        let (input, eop) = be_u8(input)?;

        let checksum_packet_len = 2+1+1+2+1+payload_size as usize;
        let checksum_calculated = Self::get_checksum(&stream[0..checksum_packet_len]);

        if magic != 0xfe55 && src != 0x14 && dst != 0x64 && cmd!=wanted_cmd && checksum_calculated != checksum && eop!=0xae {
            return IResult::Err(Err::Error(ParseError::from_error_kind(input, ErrorKind::Verify)))
        } else {
            return IResult::Ok( (payload, &[]) );
        }
    }

    pub fn parse_answer(&self, stream: &mut TcpStream) -> Result<Vec<FieldApp>, String> {
        let mut buf: [u8; 1024] = [0; 1024];
        let wanted_cmd = hexadecimal_u16_value(&self.cmd).unwrap().1;
        let mut vec_res: Vec<FieldApp> = vec![];
        match stream.read(&mut buf) {
            Ok(_buf_read) => {
                // println!("# Answer:\n\n{:x?}\n", &buf[0.._buf_read]);
                let r = self.parse_sermatec_packet(wanted_cmd, &buf);
                match r {
                    Ok( (input, _) ) => {
                        let mut order = 0;
                        let mut input = input;
                        for field in &self.fields {
                            if field.order < order {
                                return Err(format!("JSON Error! fields not sorted for {} command", wanted_cmd));
                            }
                            order = field.order;
                            let (input2, fieldtype ) = match field.parse(input) {
                                Ok(v) => v,
                                Err(_e) => return Err(format!("Command {:x}, Field {}, Parsing error", wanted_cmd, order)),
                            };
                            // Debug:
                            // println!("tag:{}, name:{}, unit:{}, value:{:?}", tag, name, _unit, value);
                            let fieldapp = FieldApp::new(field, fieldtype);
                            vec_res.push( fieldapp );
                            input = input2;
                        }
                    },
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
        return Ok(vec_res);
    }

}


