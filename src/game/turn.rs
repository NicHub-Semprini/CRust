use std::fmt::{Display, Formatter};

use crate::game::play::FullMove;

pub struct Turn {
    index: u8,
    moves: FullMove,
}

impl Turn {
    pub fn new(index: u8, moves: FullMove) -> Turn {
        Turn { index, moves }
    }

    pub fn get_index(&self) -> u8 {
        self.index
    }

    pub fn get_moves(&self) -> &FullMove {
        &self.moves
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}. {}", self.index, self.moves))
    }
}
