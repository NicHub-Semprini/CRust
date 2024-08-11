use crate::game::play::HalfMove;
use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::color::Colour;
use crate::model::piece::piece::{Piece, PieceType};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Queen {
    colour: Colour,
}

impl Queen {
    pub fn new(colour: Colour) -> Queen {
        Queen { colour }
    }
}

impl Piece for Queen {
    fn get_type(&self) -> PieceType {
        PieceType::Queen(self.clone())
    }

    fn get_symbol(&self) -> char {
        'Q'
    }

    fn get_value(&self) -> u8 {
        9
    }

    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn set_first_move(&mut self, _value: bool) -> () {}

    fn available_moves(&self, board: &Board, position: &Square) -> Vec<HalfMove> {
        let mut moves = vec![];

        // N
        moves = self.next_move_recursive(board, position, position, moves, false, 1, 0);

        // NW
        moves = self.next_move_recursive(board, position, position, moves, false, 1, -1);

        // W
        moves = self.next_move_recursive(board, position, position, moves, false, 0, -1);

        // SW
        moves = self.next_move_recursive(board, position, position, moves, false, -1, -1);

        // S
        moves = self.next_move_recursive(board, position, position, moves, false, -1, 0);

        // SE
        moves = self.next_move_recursive(board, position, position, moves, false, -1, 1);

        // E
        moves = self.next_move_recursive(board, position, position, moves, false, 0, 1);

        // NE
        moves = self.next_move_recursive(board, position, position, moves, false, 1, 1);

        moves
    }

    fn duplicate(&self) -> Box<dyn Piece> {
        Box::new(Queen::new(self.colour))
    }
}
