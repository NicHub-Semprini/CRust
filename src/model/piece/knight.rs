use crate::game::play::HalfMove;
use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::color::Colour;
use crate::model::piece::piece::{Piece, PieceType};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Knight {
    colour: Colour,
}

impl Knight {
    pub fn new(colour: Colour) -> Knight {
        Knight { colour }
    }
}

impl Piece for Knight {
    fn get_type(&self) -> PieceType {
        PieceType::Knight(self.clone())
    }

    fn get_symbol(&self) -> char {
        'N'
    }

    fn get_value(&self) -> u8 {
        3
    }

    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn set_first_move(&mut self, _value: bool) -> () {}

    fn available_moves(&self, board: &Board, position: &Square) -> Vec<HalfMove> {
        let mut moves = vec![];

        // Q1
        moves = self.next_move_single(board, position, moves, 1, 2);
        moves = self.next_move_single(board, position, moves, 2, 1);

        // Q2
        moves = self.next_move_single(board, position, moves, 2, -1);
        moves = self.next_move_single(board, position, moves, 1, -2);

        // Q3
        moves = self.next_move_single(board, position, moves, -1, -2);
        moves = self.next_move_single(board, position, moves, -2, -1);

        // Q4
        moves = self.next_move_single(board, position, moves, -2, 1);
        moves = self.next_move_single(board, position, moves, -1, 2);

        moves
    }

    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Knight::new(self.colour))
    }
}
