use super::command::Command;
use super::field::Field;
use super::fieldtype::FieldType;

pub struct FieldApp<'a> {
    pub c: &'a Command,
    pub f: &'a Field,
    pub v: FieldType
}

impl<'a> FieldApp<'a> {
    pub fn new(c: &'a Command, f: &'a Field, v: FieldType) -> Self {
        FieldApp {
            c: c,
            f: f,
            v: v,
        }
    }

}

impl<'a> FieldApp<'a> {

    /// Convert consts to human readable text
    fn expand_fmt(&'a self) -> Option<&str>  {
        match self.c.cmd.as_str() {
            "0A" => {
                match self.f.name.as_str() {
                    "Charge and discharge status" => {
                        let t = match self.v {
                            FieldType::Int(0x11) => "Charge",
                            FieldType::Int(0x22) => "Discharge",
                            FieldType::Int(0x33) => "Standby",
                            _ => "Unknown",
                        };
                        Some(t)
                    },
                    "Battery communication connection status" => {
                        let t = match self.v {
                            FieldType::UInt(0) => "Normal",
                            FieldType::UInt(1) => "Battery Communication Disconnected",
                            _ => "Unknown",
                        };
                        Some(t)
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }
}

impl std::fmt::Debug for FieldApp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if self.f.tag.len() != 0 {
            format!("{} ({})", self.f.name, self.f.tag)
        } else {
            format!("{}", self.f.name)
        };

        let s = match self.expand_fmt() {
            Some(v) => {
                format!("{}: {}", s, v)
            },
            None => {
                match self.f.get_unit() {
                    Some(unit) => {
                        let unit_v = match self.v {
                            FieldType::Int(w) => Some(w as f32 * unit),
                            FieldType::Long(w) => Some(w as f32 * unit),
                            FieldType::UInt(w) => Some(w as f32 * unit),
                            _ => None,
                        };
                        match unit_v {
                            Some(v) => format!("{}: {:.3}", s, v),
                            None => format!("{}: {:?}", s, self.v),
                        }
                    },
                    None => format!("{}: {:?}", s, self.v),
                }
            },
        };

        let s = if self.f.unit_type.len() != 0 {
            format!("{} {}", s, self.f.unit_type)
        } else {
            s
        };

        write!(f, "{}", s)
    }
}