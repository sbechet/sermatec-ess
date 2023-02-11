use super::Field;
use super::fieldtype::FieldType;

pub struct FieldApp<'a> {
    pub f: &'a Field,
    pub v: FieldType
}

impl<'a> FieldApp<'a> {
    pub fn new(f: &'a Field, v: FieldType) -> Self {
        FieldApp {
            f: f,
            v: v,
        }
    }
}

impl std::fmt::Debug for FieldApp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.f.tag.len() != 0 {
            write!(f, "{} ({}): {:?}", self.f.name, self.f.tag, self.v)
        } else {
            write!(f,"{}: {:?}", self.f.name, self.v)
        }
    }
}