use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use colored::Colorize;
use enum_dispatch::enum_dispatch;

use crate::game::play::HalfMove;
use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::bishop::Bishop;
use crate::model::piece::color::Colour;
use crate::model::piece::king::King;
use crate::model::piece::knight::Knight;
use crate::model::piece::pawn::Pawn;
use crate::model::piece::queen::Queen;
use crate::model::piece::rook::Rook;

#[enum_dispatch(Piece)]
#[derive(Debug, Copy, Clone, Hash)]
pub enum PieceType {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        self.get_symbol() == other.get_symbol()
    }
}

#[enum_dispatch]
pub trait Piece: Debug {
    fn get_type(&self) -> PieceType;

    fn get_symbol(&self) -> char;

    fn get_value(&self) -> u8;

    fn get_colour(&self) -> Colour;

    fn set_first_move(&mut self, value: bool) -> ();

    fn available_moves(&self, board: &Board, position: &Square) -> Vec<HalfMove>;

    fn can_move(&self, board: &Board, position: &Square) -> bool {
        !self.available_moves(board, position).is_empty()
    }

    fn duplicate(&self) -> Box<dyn Piece>;

    fn capture(
        &self,
        board: &Board,
        position: &Square,
        target_rank: i8,
        target_line: i8,
    ) -> Option<HalfMove> {
        match board.get_square(target_rank, target_line) {
            Some(square) => {
                if self.can_capture(board, square) {
                    // Promotion (only for pawns)
                    let promotion = self.get_type()
                        == PieceType::Pawn(Pawn::new(self.get_colour()))
                        && board.is_promotion_square(square, self.get_colour());

                    // Add move
                    Some(HalfMove::new(
                        self.get_type(),
                        position.clone(),
                        square.clone(),
                        true,
                        promotion,
                        false,
                        false,
                        false,
                        0.0,
                    ))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn can_capture(&self, board: &Board, target: &Square) -> bool {
        match board.get_piece(target.clone(), Some(self.get_colour().get_opposite())) {
            Some(_) => true,
            None => false,
        }
    }

    fn is_controlling(&self, board: &Board, position: &Square, target: &Square) -> bool {
        for m in self.available_moves(board, position) {
            if m.get_to() == *target {
                return true;
            }
        }
        false
    }

    fn next_move_recursive(
        &self,
        board: &Board,
        original_position: &Square,
        position: &Square,
        mut moves: Vec<HalfMove>,
        has_captured: bool,
        step_r: i8,
        step_f: i8,
    ) -> Vec<HalfMove> {
        // Base case: last move was a capture
        if has_captured {
            return moves;
        }

        let rank = position.get_rank() as i8;
        let file = position.get_file() as i8;
        return match board.get_square(rank + step_r, file + step_f) {
            Some(target) => {
                let can_capture = self.can_capture(board, target);
                if board.is_square_free(&target) || can_capture {
                    // Add move
                    moves.push(HalfMove::new(
                        self.get_type(),
                        original_position.clone(),
                        target.clone(),
                        can_capture,
                        false,
                        false,
                        false,
                        false,
                        0.0,
                    ));

                    // Go to next move
                    return self.next_move_recursive(
                        board,
                        original_position,
                        target,
                        moves,
                        can_capture,
                        step_r,
                        step_f,
                    );
                }

                // Base case: target square is not free or is hidden by another piece
                moves
            }

            // Base case: end of board reached
            None => moves,
        };
    }

    fn next_move_single(
        &self,
        board: &Board,
        position: &Square,
        mut moves: Vec<HalfMove>,
        step_r: i8,
        step_f: i8,
    ) -> Vec<HalfMove> {
        let rank = position.get_rank() as i8;
        let file = position.get_file() as i8;
        return match board.get_square(rank + step_r, file + step_f) {
            Some(square) => {
                if board.is_square_free(square) || self.can_capture(board, square) {
                    // Add move
                    moves.push(HalfMove::new(
                        self.get_type(),
                        position.clone(),
                        square.clone(),
                        self.can_capture(board, square),
                        false,
                        false,
                        false,
                        false,
                        0.0,
                    ));
                }
                moves
            }

            // Base case: end of board reached
            None => moves,
        };
    }
}

impl Display for dyn Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let piece_type = format!("{}", self.get_symbol());
        match self.get_colour() {
            Colour::White => f.write_str(&format!("{}", piece_type.bold().blue())),
            Colour::Black => f.write_str(&format!("{}", piece_type.bold())),
        }
    }
}
