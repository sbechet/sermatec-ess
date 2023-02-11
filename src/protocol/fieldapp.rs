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
        let s = if self.f.tag.len() != 0 {
            format!("{} ({})", self.f.name, self.f.tag)
        } else {
            format!("{}", self.f.name)
        };

        let s = format!("{}: {:?}", s, self.v);

        let s = if self.f.unit_type.len() != 0 {
            format!("{} {}", s, self.f.unit_type)
        } else {
            s
        };

        write!(f, "{}", s)
    }
}