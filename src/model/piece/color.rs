use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Colour {
    Black,
    White,
}

impl Colour {
    pub fn is_same(&self, other: &Colour) -> bool {
        self == other
    }

    pub fn is_opposite(&self, other: &Colour) -> bool {
        !self.is_same(other)
    }

    pub fn get_opposite(&self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::White => f.write_str("white"),
            Colour::Black => f.write_str("black"),
        }
    }
}
