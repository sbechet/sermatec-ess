use super::fieldtype::FieldType;
use super::Protocol;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Hardware {
    pub model_code: String,
    pub pcu_version: i16,
    pub pcu_version_s: String,
    pub product_sn: String,
    pub product_sn_ln: String,
    pub battery_name: String,
}

impl Hardware {
    pub fn get_real_pcu_version(v: i32) -> i32 {
        if v == 991 || v == 998 {
            601
        } else {
            v
        }
    }

    pub fn get_info(p: &HashMap<String, Protocol>, stream: &mut TcpStream) -> Option<Hardware> {
        let command = p["osim"].get_command(0, "98").unwrap();
        let packet = command.build_packet().unwrap();
        stream.write(&packet).unwrap();

        let elements = command.parse_answer(stream);
        let mut model_code: String = String::from("");
        let mut pcu_version: i16 = 0;
        let mut pcu_version_s: String = String::from("");
        let mut product_sn: String = String::from("");
        let mut product_sn_ln: String = String::from("");
        let mut battery_name: String = String::from("");
        match &elements {
            Ok(elts) => {
                for fa in elts {
                    if fa.f.tag == "pcuVersion" {
                        pcu_version = if let FieldType::Int(v) = fa.v {
                            Self::get_real_pcu_version(v) as i16
                        } else {
                            0 as i16
                        };
                        pcu_version_s = format!("{:?}", fa);
                    }
                    if fa.f.name == "product_sn" {
                        if let FieldType::String(s) = &fa.v {
                            product_sn = s.to_string();
                        }
                    }
                    if fa.f.name == "product_sn_ln" {
                        if let FieldType::String(s) = &fa.v {
                            product_sn_ln = s.to_string();
                        }
                    }
                    if fa.f.name == "Battery manufacturer number (code list)" {
                        battery_name = format!("{:?}", fa);
                    }
                    if fa.f.name == "model code" {
                        model_code = format!("{:?}", fa);
                    }
                }
            }
            Err(e) => {
                println!("Parsing Error: {:?}", e);
                return None;
            }
        };

        Some(Hardware {
            model_code: model_code,
            pcu_version: pcu_version,
            pcu_version_s: pcu_version_s,
            product_sn: product_sn,
            product_sn_ln: product_sn_ln,
            battery_name: battery_name,
        })
    }
}

impl std::fmt::Debug for Hardware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "model_code: {}\npcu_version: {}\nproduct_sn: {}\nproduct_sn_ln: {}\nbattery_name: {}\n", self.model_code, self.pcu_version_s, self.product_sn, self.product_sn_ln, self.battery_name)
    }
}
