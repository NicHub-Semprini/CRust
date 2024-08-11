use crate::game::play::HalfMove;
use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::color::Colour;
use crate::model::piece::piece::{Piece, PieceType};

#[derive(Debug, Copy, Clone, Hash)]
pub struct King {
    colour: Colour,
    first_move: bool,
}

impl King {
    pub fn new(colour: Colour) -> King {
        King {
            colour,
            first_move: true,
        }
    }
}

impl Piece for King {
    fn get_type(&self) -> PieceType {
        PieceType::King(self.clone())
    }

    fn get_symbol(&self) -> char {
        'K'
    }

    fn get_value(&self) -> u8 {
        u8::MAX
    }

    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn set_first_move(&mut self, value: bool) -> () {
        self.first_move = value;
    }

    fn available_moves(&self, board: &Board, position: &Square) -> Vec<HalfMove> {
        let mut moves = vec![];

        // Standard moves
        // N
        moves = self.next_move_single(board, position, moves, 1, 0);

        // NW
        moves = self.next_move_single(board, position, moves, 1, -1);

        // W
        moves = self.next_move_single(board, position, moves, 0, -1);

        // SW
        moves = self.next_move_single(board, position, moves, -1, -1);

        // S
        moves = self.next_move_single(board, position, moves, -1, 0);

        // SE
        moves = self.next_move_single(board, position, moves, -1, 1);

        // E
        moves = self.next_move_single(board, position, moves, 0, 1);

        // NE
        moves = self.next_move_single(board, position, moves, 1, 1);

        // Special moves
        // 1. Castling
        // TODO

        moves
    }

    fn duplicate(&self) -> Box<dyn Piece> {
        let mut new = Box::new(King::new(self.colour));
        new.set_first_move(self.first_move);
        new
    }
}
