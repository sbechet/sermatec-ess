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

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub order: u8,
    #[serde(rename="byteLen")]
    pub byte_len: u8,
    #[serde(rename="byteOrder")]
    #[serde(default)]
    pub byte_order: u8,
    #[serde(default)]
    pub tag: String,
    #[serde(rename="type")]
    pub type_type: String,
    pub name: String,
    #[serde(default)]
    pub converter: String,
    #[serde(default)]
    pub validate: String,
    #[serde(rename="unitType")]
    #[serde(default)]
    pub unit_type: String,
    #[serde(rename="unitValue")]
    #[serde(default)]
    pub unit_value: String,
    #[serde(default)]
    pub precision: u8,

    #[serde(default)]
    group: u8,
    #[serde(rename="groupTag")]
    #[serde(default)]
    group_tag: String,

    #[serde(default)]
    repeat: u8,
    #[serde(rename="repeatRef")]
    #[serde(default)]
    repeat_ref: u8,
    #[serde(rename="repeatGroup")]
    #[serde(default)]
    repeat_group: u8,


    #[serde(rename="fromBit")]
    #[serde(default)]
    from_bit: u8,
    #[serde(rename="endBit")]
    #[serde(default)]
    end_bit: u8,
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    #[serde(rename="bitPosition")]
    #[serde(default)]
    bit_position: u8,

    #[serde(default)]
    same: bool,

    #[serde(rename="defaultValue")]
    #[serde(default)]
    default_value: String,

    #[serde(rename="return")]
    #[serde(default)]
    return_value: String,
}

pub enum FieldType {
    // Bit(bool),
    Int(i64),
    // BitRange(Vec<bool>),
    // Bytes(Vec<u8>),
    // Hex(i16),
    Long(i64),
    // OnePosiiton(String),
    // Preserve(Vec<u8>),
    String(String),
    UInt(u16),
}

impl std::fmt::Debug for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match &self {
            FieldType::Int(i) => format!("{}", i),
            FieldType::Long(i) => format!("{}", i),
            FieldType::String(s) => format!("{}", s),
            FieldType::UInt(u) => format!("{}", u),
        };
        write!(f, "{}", t)
    }
}

impl Protocol {
    pub fn new() -> HashMap<String, Protocol> {
        let protocol_str = String::from_utf8_lossy(PROTOCOL);
        let protocol: serde_json::Result<HashMap<String, Protocol>> = serde_json::from_str(&protocol_str);
        protocol.unwrap()
    }

    pub fn listing(&self, current_version: i16) {
        let cmds = self.get_commands(current_version);


        println!("XXX TESTED AND WORKING FOR ME: 98 0A 0B 0D 99 9A");
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


    pub fn print_nice_answer(&self, answer: &Result<Vec<(String, String, FieldType)>, String>) {
        match answer {
            Ok(a) => {
                for e in a {
                    if e.0.len() != 0 {
                        println!("{} ({}): {:?}", e.1, e.0, e.2);
                    } else {
                        println!("{}: {:?}", e.1, e.2);
                    }
                    
                }
                println!();
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    fn parse_sermatec_packet<'a>(&'a self, wanted_cmd: u16, stream: &'a [u8]) -> IResult<&[u8],&[u8]> {
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

    pub fn parse_answer(&self, stream: &mut TcpStream) -> Result<Vec<(String, String, FieldType)>, String> {
        let mut buf: [u8; 1024] = [0; 1024];
        let wanted_cmd = hexadecimal_u16_value(&self.cmd).unwrap().1;
        let mut vec_res: Vec<(String, String, FieldType)> = vec![];
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
                            let (input2, (tag, name, _unit, value) ) = match field.parse(input) {
                                Ok(v) => v,
                                Err(_e) => return Err(format!("Command {:x}, Field {}, Parsing error", wanted_cmd, order)),
                            };
                            // Debug:
                            // println!("tag:{}, name:{}, unit:{}, value:{:?}", tag, name, _unit, value);
                            vec_res.push( (tag.to_string(), name.to_string(), value) );
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


impl Field {
    // (tag, name, unit, value)
    pub fn parse<'a>(&'a self, input: &'a [u8]) -> IResult<&'a [u8], (&'a str, &'a str, &'a str, FieldType)> {
        let (input, value) = match self.type_type.as_str() {
            // "bit" => FieldType::Bit(bytes[0] == 1),
            "int" => {
                let (input, value) = match self.byte_len {
                    1 => {
                        let (input, value) = be_i8(input)?;
                        (input, value as i64)
                    },
                    2 => {
                        let (input, value) = be_i16(input)?;
                        (input, value as i64)
                    },
                    4 => {
                        let (input, value) = be_i64(input)?;
                        (input, value)
                    },
                    _ => (input, 0),
                };
                // let (input, value) = be_i16(input)?;
                (input, Some(FieldType::Int(value)))
            },
            // "bitRange" => {
            //     let v = u16::from_be_bytes(bytes);
            //     FieldType::BitRange(Vec<bool>);
            // },
            // "bytes" => Bytes(Vec<u8>),
            // "hex" => Hex(i16),
            "long" => {
                let (input, value) = be_i64(input)?;
                (input, Some(FieldType::Long(value)))
            },
            // "onePosition" => OnePosiiton(String),
            // "preserve" => Preserve(Vec<u8>),
            "string" => {
                let (input, v) = take(self.byte_len as usize)(input)?;
                let s = String::from_utf8(v.to_vec()).unwrap();
                (input, Some(FieldType::String(s)))
            },
            "uInt" => {
                let (input, value) = be_u16(input)?;
                (input, Some(FieldType::UInt(value)))
            },
            _ => (input, None),
        };
        return match value {
            Some(v) => Ok( (input, (self.tag.as_str(), self.name.as_str(), self.unit_type.as_str(), v)) ),
            None => IResult::Err(Err::Error(ParseError::from_error_kind(input, ErrorKind::Verify)))
        };
    }
}   
