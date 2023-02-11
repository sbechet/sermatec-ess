use nom::IResult;
use nom::bytes::complete::*;
use nom::number::complete::*;
use nom::{Err, error::ErrorKind, error::ParseError};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, PickFirst};

use super::fieldtype::FieldType;

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

impl Field {

    fn get_unit(&self) -> f64 {
        match self.unit_value.as_str() {
            "0.001" => 0.001,
            "0.01" => 0.01,
            "0.1" => 0.1,
            _ => 1.0,
        }
    }

    pub fn parse<'a>(&'a self, input: &'a [u8]) -> IResult<&'a [u8], FieldType> {
        let (input, value) = match self.type_type.as_str() {
            // "bit" => FieldType::Bit(bytes[0] == 1),
            "int" => {
                let (input, value) = match self.byte_len {
                    1 => {
                        let (input, value) = be_i8(input)?;
                        (input, value as i32)
                    },
                    2 => {
                        let (input, value) = be_i16(input)?;
                        (input, value as i32)
                    },
                    4 => {
                        let (input, value) = be_i32(input)?;
                        (input, value as i32)
                    },
                    _ => (input, 0),
                };
                let value = value as f64 * self.get_unit();
                (input, Some(FieldType::Int(value)))
            },
            // "bitRange" => {
            //     let v = u16::from_be_bytes(bytes);
            //     FieldType::BitRange(Vec<bool>);
            // },
            // "bytes" => Bytes(Vec<u8>),
            "hex" => {
                let (input, value) = be_u16(input)?;
                // TODO: One day use converter field
                // let converter = self.converter;
                let value = match value {
                    0xee00 => 1,
                    0x00ee => 2,
                    _ => 0,
                };
                (input, Some(FieldType::Hex(value)))
            },
            "long" => {
                let (input, value) = be_i32(input)?;
                let value = value as f64 * self.get_unit();
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
                let value = value as f64 * self.get_unit();
                (input, Some(FieldType::UInt(value)))
            },
            _ => (input, None),
        };
        return match value {
            Some(v) => Ok( (input, v) ),
            None => IResult::Err(Err::Error(ParseError::from_error_kind(input, ErrorKind::Verify)))
        };
    }
}   
