use crate::game::play::HalfMove;
use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::color::Colour;
use crate::model::piece::piece::{Piece, PieceType};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Rook {
    colour: Colour,
    first_move: bool,
}

impl Rook {
    pub fn new(colour: Colour) -> Rook {
        Rook {
            colour,
            first_move: true,
        }
    }
}

impl Piece for Rook {
    fn get_type(&self) -> PieceType {
        PieceType::Rook(self.clone())
    }

    fn get_symbol(&self) -> char {
        'R'
    }

    fn get_value(&self) -> u8 {
        5
    }

    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn set_first_move(&mut self, value: bool) -> () {
        self.first_move = value;
    }

    fn available_moves(&self, board: &Board, position: &Square) -> Vec<HalfMove> {
        let mut moves = vec![];

        // N
        moves = self.next_move_recursive(board, position, position, moves, false, 1, 0);

        // W
        moves = self.next_move_recursive(board, position, position, moves, false, 0, -1);

        // S
        moves = self.next_move_recursive(board, position, position, moves, false, -1, 0);

        // E
        moves = self.next_move_recursive(board, position, position, moves, false, 0, 1);

        moves
    }

    fn duplicate(&self) -> Box<dyn Piece> {
        let mut new = Box::new(Rook::new(self.colour));
        new.set_first_move(self.first_move);
        new
    }
}
