use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Part {
    P1 = 1,
    P2 = 2,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Id {
    pub year: u16,
    pub day: u8,
    pub part: Part,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} p{}", self.year, self.day, self.part)
    }
}
