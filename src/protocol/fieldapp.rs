use super::command::Command;
use super::field::Field;
use super::fieldtype::FieldType;

use super::hardware::Hardware;

pub struct FieldApp<'a> {
    pub c: &'a Command,
    pub f: &'a Field,
    pub v: FieldType,
}

impl<'a> FieldApp<'a> {
    pub fn new(c: &'a Command, f: &'a Field, v: FieldType) -> Self {
        FieldApp { c: c, f: f, v: v }
    }
}

impl<'a> FieldApp<'a> {
    /// Convert consts to human readable text
    fn expand_fmt(&'a self) -> Option<&str> {
        match self.c.cmd.as_str() {
            "0A" => match self.f.name.as_str() {
                "Charge and discharge status" => {
                    let t = match self.v {
                        FieldType::Int(0x11) => "Charge",
                        FieldType::Int(0x22) => "Discharge",
                        FieldType::Int(0x33) => "Standby",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                "Battery communication connection status" => {
                    let t = match self.v {
                        FieldType::UInt(0) => "Normal",
                        FieldType::UInt(1) => "Battery Communication Disconnected",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                _ => None,
            },
            "95" => match self.f.name.as_str() {
                "Operating mode" => {
                    let t = match self.v {
                        FieldType::Int(1) => "General",
                        FieldType::Int(2) => "Energy Storage",
                        FieldType::Int(3) => "Micro-grid",
                        FieldType::Int(4) => "Peak-Valley",
                        FieldType::Int(5) => "AC coupling",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                "anti-backflow function" => {
                    let t = match self.v {
                        FieldType::Hex(0xee00) => "True",
                        FieldType::Hex(0x00ee) => "False",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                "grid code" => {
                    let t = match self.v {
                        FieldType::Int(1) => "China",
                        FieldType::Int(2) => "Australia",
                        FieldType::Int(3) => "Germany",
                        FieldType::Int(4) => "England",
                        FieldType::Int(5) => "Italy",
                        FieldType::Int(6) => "French",
                        FieldType::Int(7) => "Austria",
                        FieldType::Int(8) => "SouthAfrica",
                        FieldType::Int(9) => "Czech",
                        FieldType::Int(10) => "Belgium",
                        FieldType::Int(11) => "Swiss",
                        FieldType::Int(12) => "The Philippines",
                        FieldType::Int(13) => "Default 50Hz Grid",
                        FieldType::Int(14) => "Default 60Hz Grid",
                        FieldType::Int(15) => "Thailand",
                        FieldType::Int(16) => "Denmark",
                        FieldType::Int(17) => "Greece",
                        FieldType::Int(18) => "The Netherlands",
                        FieldType::Int(19) => "India",
                        FieldType::Int(20) => "Ireland",
                        FieldType::Int(21) => "New Zealand",
                        FieldType::Int(22) => "Poland",
                        FieldType::Int(23) => "Spain",
                        FieldType::Int(24) => "Portugal",
                        FieldType::Int(25) => "Pakistan",
                        FieldType::Int(26) => "Custom",
                        FieldType::Int(27) => "Ukraine",
                        FieldType::Int(28) => "Slovenia",
                        FieldType::Int(29) => "Vietnam",
                        FieldType::Int(30) => "Finland",
                        FieldType::Int(31) => "Norway",
                        FieldType::Int(32) => "Sweden",
                        FieldType::Int(33) => "Botswana",
                        FieldType::Int(34) => "Angola",
                        FieldType::Int(35) => "Zimbabwe",
                        FieldType::Int(36) => "Tanzania",
                        FieldType::Int(37) => "Nigeria",
                        FieldType::Int(38) => "Kenya",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                "DC side battery type" => {
                    let t = match self.v {
                        FieldType::Int(1) => "Lithium Battery",
                        FieldType::Int(2) => "Lead-acid Battery",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                "Battery communication protocol selection" => {
                    let t = match self.v {
                        FieldType::Int(1) => "No Battery",
                        FieldType::Int(2) => "PylonTech High Voltage Battery",
                        FieldType::Int(3) => "PylonTech Low Voltage Battery",
                        FieldType::Int(9) => "Nelumbo HV Battery",
                        FieldType::Int(12) => "Default High Voltage Battery",
                        FieldType::Int(13) => "Default Low Voltage Battery",
                        FieldType::Int(14) => "Dyness High Voltage Battery",
                        FieldType::Int(22) => "BYD High Voltage Battery",
                        FieldType::Int(23) => "BYD Low Voltage Battery",
                        FieldType::Int(25) => "AOBO Battery",
                        FieldType::Int(26) => "Soluna 15K Pack HV",
                        FieldType::Int(27) => "Soluna 4K LV",
                        FieldType::Int(28) => "Soluna 3K LV",
                        FieldType::Int(30) => "PYLON Low-voltage Battery 485",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                "Meter communication protocol selection" => {
                    let t = match self.v {
                        FieldType::Int(1) => "No Electric Meter",
                        FieldType::Int(2) => "Acrel Three-phases meter",
                        FieldType::Int(3) => "Acrel Single-phase meter",
                        FieldType::Int(4) => "Three-phases Eastron meter",
                        FieldType::Int(5) => "Single-phase Eastron meter",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                _ => None,
            },
            "98" => match self.f.name.as_str() {
                "Battery manufacturer number (code list)" => {
                    let t = match self.v {
                        FieldType::Int(1) => "No Battery",
                        FieldType::Int(2) => "PylonTech High Voltage Battery",
                        FieldType::Int(3) => "PylonTech Low Voltage Battery",
                        FieldType::Int(9) => "Nelumbo HV Battery",
                        FieldType::Int(12) => "Default High Voltage Battery",
                        FieldType::Int(13) => "Default Low Voltage Battery",
                        FieldType::Int(14) => "Dyness High Voltage Battery",
                        FieldType::Int(22) => "BYD High Voltage Battery",
                        FieldType::Int(23) => "BYD Low Voltage Battery",
                        FieldType::Int(25) => "AOBO Battery",
                        FieldType::Int(26) => "Soluna 15K Pack HV",
                        FieldType::Int(27) => "Soluna 4K LV",
                        FieldType::Int(28) => "Soluna 3K LV",
                        FieldType::Int(30) => "PYLON Low-voltage Battery 485",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                "model code" => {
                    let t = match self.v {
                        FieldType::Int(1) => "10kW",
                        FieldType::Int(2) => "5kW",
                        FieldType::Int(3) => "6kW",
                        FieldType::Int(5) => "3kW",
                        _ => "Unknown",
                    };
                    Some(t)
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl std::fmt::Debug for FieldApp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.expand_fmt() {
            Some(v) => {
                format!("{}", v)
            }
            None => match self.f.get_unit() {
                Some(unit) => {
                    let unit_v = match self.v {
                        FieldType::Int(w) => Some(w as f32 * unit),
                        FieldType::Long(w) => Some(w as f32 * unit),
                        FieldType::UInt(w) => Some(w as f32 * unit),
                        _ => None,
                    };
                    match unit_v {
                        Some(v) => format!("{:.3}", v),
                        None => format!("{:?}", self.v),
                    }
                }
                None => {
                    if let "98" = self.c.cmd.as_str() {
                        if let "protocol version number" = self.f.name.as_str() {
                            let pcu_version = if let FieldType::Int(v) = self.v {
                                Hardware::get_real_pcu_version(v) as i16
                            } else {
                                0 as i16
                            };
                            format!(
                                "{}.{}.{}",
                                (pcu_version / 100) % 10,
                                (pcu_version / 10) % 10,
                                pcu_version % 10
                            )
                        } else {
                            format!("{:?}", self.v)
                        }
                    } else {
                        format!("{:?}", self.v)
                    }
                }
            },
        };

        write!(f, "{}", s)
    }
}
