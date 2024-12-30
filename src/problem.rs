#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Part {
    P1 = 1,
    P2 = 2,
}

impl TryInto<Part> for u8 {
    type Error = ();
    fn try_into(self) -> Result<Part, Self::Error> {
        match self {
            1 => Ok(Part::P1),
            2 => Ok(Part::P2),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Id {
    pub year: u16,
    pub day: u8,
    pub part: Part,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:_>2} {}", self.year, self.day, self.part)
    }
}
