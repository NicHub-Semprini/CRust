use crate::game::play::HalfMove;
use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::color::Colour;
use crate::model::piece::piece::{Piece, PieceType};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Bishop {
    colour: Colour,
}

impl Bishop {
    pub fn new(colour: Colour) -> Bishop {
        Bishop { colour }
    }
}

impl Piece for Bishop {
    fn get_type(&self) -> PieceType {
        PieceType::Bishop(self.clone())
    }

    fn get_symbol(&self) -> char {
        'B'
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

        // NE diagonal
        moves = self.next_move_recursive(board, position, position, moves, false, 1, 1);

        // NW diagonal
        moves = self.next_move_recursive(board, position, position, moves, false, 1, -1);

        // SW diagonal
        moves = self.next_move_recursive(board, position, position, moves, false, -1, -1);

        // SE diagonal
        moves = self.next_move_recursive(board, position, position, moves, false, -1, 1);

        moves
    }

    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Bishop::new(self.colour))
    }
}
