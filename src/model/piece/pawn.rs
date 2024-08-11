use crate::game::play::HalfMove;
use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::color::Colour;
use crate::model::piece::piece::{Piece, PieceType};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Pawn {
    colour: Colour,
    first_move: bool,
}

impl Pawn {
    pub fn new(colour: Colour) -> Pawn {
        Pawn {
            colour,
            first_move: true,
        }
    }

    fn get_increment(&self) -> i8 {
        match self.colour {
            Colour::White => 1,
            Colour::Black => -1,
        }
    }

    fn next_move(
        &self,
        board: &Board,
        original_position: &Square,
        position: &Square,
        mut moves: Vec<HalfMove>,
        it: u8,
        max_it: u8,
        step_r: i8,
        step_f: i8,
    ) -> Vec<HalfMove> {
        // Base case: 2 steps done
        if it > max_it {
            return moves;
        }

        let rank = position.get_rank() as i8;
        let file = position.get_file() as i8;
        return match board.get_square(rank + step_r, file + step_f) {
            Some(square) => {
                if board.is_square_free(square) {
                    // Promotion
                    let promotion = board.is_promotion_square(square, self.get_colour());

                    // Add move
                    moves.push(HalfMove::new(
                        self.get_type(),
                        original_position.clone(),
                        square.clone(),
                        false,
                        promotion,
                        false,
                        false,
                        false,
                        0.0,
                    ));

                    // Go to next move
                    return self.next_move(
                        board,
                        original_position,
                        square,
                        moves,
                        it + 1,
                        max_it,
                        step_r,
                        step_f,
                    );
                }

                // Base case: target square is not free
                moves
            }

            // Base case: end of board reached
            None => moves,
        };
    }

    fn capture_left(&self, board: &Board, position: &Square) -> Option<HalfMove> {
        let target_rank = (position.get_rank() as i8) + self.get_increment();
        let target_line = (position.get_file() as i8) - 1;
        self.capture(board, position, target_rank, target_line)
    }

    fn capture_right(&self, board: &Board, position: &Square) -> Option<HalfMove> {
        let target_rank = (position.get_rank() as i8) + self.get_increment();
        let target_line = (position.get_file() as i8) + 1;
        self.capture(board, position, target_rank, target_line)
    }
}

impl Piece for Pawn {
    fn get_type(&self) -> PieceType {
        PieceType::Pawn(self.clone())
    }

    fn get_symbol(&self) -> char {
        'P'
    }

    fn get_value(&self) -> u8 {
        1
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
        let max_it = match self.first_move {
            true => 2,
            false => 1,
        };
        moves = self.next_move(
            board,
            position,
            position,
            moves,
            1,
            max_it,
            self.get_increment(),
            0,
        );

        // Special moves
        // 1. Capture
        match self.capture_left(board, position) {
            Some(half_move) => moves.push(half_move),
            None => {}
        }
        match self.capture_right(board, position) {
            Some(half_move) => moves.push(half_move),
            None => {}
        }

        // 2. En-passant
        // TODO

        moves
    }

    fn duplicate(&self) -> Box<dyn Piece> {
        let mut new = Box::new(Pawn::new(self.colour));
        new.set_first_move(self.first_move);
        new
    }
}
