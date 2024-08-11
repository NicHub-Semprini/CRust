use std::fmt::{Debug, Display, Formatter};

use crate::utils::{normalize_file, normalize_rank};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Square {
    rank: u8,
    file: u8,
}

impl Square {
    pub fn new(rank: u8, file: u8) -> Square {
        Square { rank, file }
    }

    pub fn get_rank(&self) -> u8 {
        self.rank
    }

    pub fn get_file(&self) -> u8 {
        self.file
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}{}",
            normalize_file(self.file),
            normalize_rank(self.rank)
        ))
    }
}
