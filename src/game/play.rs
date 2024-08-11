use std::fmt::{Display, Formatter};

use crate::model::board::square::Square;
use crate::model::piece::piece::Piece;
use crate::model::piece::piece::PieceType;
use crate::utils;

#[derive(Debug, Copy, Clone)]
pub struct HalfMove {
    piece: PieceType,
    from: Square,
    to: Square,
    capture: bool,
    promotion: bool,
    check: bool,
    checkmate: bool,
    stalemate: bool,
    evaluation: f32,
}

impl HalfMove {
    pub fn new(
        piece: PieceType,
        from: Square,
        to: Square,
        capture: bool,
        promotion: bool,
        check: bool,
        checkmate: bool,
        stalemate: bool,
        evaluation: f32,
    ) -> HalfMove {
        HalfMove {
            piece,
            from,
            to,
            capture,
            promotion,
            check,
            checkmate,
            stalemate,
            evaluation,
        }
    }

    pub fn get_piece(&self) -> PieceType {
        self.piece
    }

    pub fn get_from(&self) -> Square {
        self.from
    }

    pub fn get_to(&self) -> Square {
        self.to
    }

    pub fn is_capture(&self) -> bool {
        self.capture
    }

    pub fn is_promotion(&self) -> bool {
        self.promotion
    }

    pub fn is_check(&self) -> bool {
        self.check
    }

    pub fn is_checkmate(&self) -> bool {
        self.checkmate
    }

    pub fn is_stalemate(&self) -> bool {
        self.stalemate
    }

    pub fn get_evaluation(&self) -> f32 {
        self.evaluation
    }

    pub fn set_promotion(&mut self, promotion: bool) -> () {
        self.promotion = promotion
    }

    pub fn set_check(&mut self, check: bool) -> () {
        self.check = check
    }

    pub fn set_checkmate(&mut self, checkmate: bool) -> () {
        self.checkmate = checkmate
    }

    pub fn set_stalemate(&mut self, stalemate: bool) -> () {
        self.stalemate = stalemate
    }

    pub fn set_evaluation(&mut self, evaluation: f32) -> () {
        self.evaluation = evaluation
    }
}

impl Display for HalfMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self.piece {
            PieceType::Pawn(_) => {
                if self.capture {
                    utils::normalize_file(self.from.get_file())
                } else {
                    '\0'
                }
            }
            _ => self.piece.get_symbol(),
        };
        let capture = match self.capture {
            true => "x",
            false => "",
        };
        let promotion = match self.promotion {
            true => "Q",
            false => "",
        };
        let check = match self.checkmate {
            true => "#",
            false => match self.check {
                true => "+",
                false => "",
            },
        };
        let stale = match self.stalemate {
            true => "(=)",
            false => "",
        };
        f.write_str(&format!(
            "{}{}{}{}{}{}",
            symbol, capture, self.to, promotion, check, stale
        ))
    }
}

#[derive(Debug)]
pub struct FullMove {
    white_move: Option<HalfMove>,
    white_seconds: f32,
    black_move: Option<HalfMove>,
    black_seconds: f32,
}

impl FullMove {
    pub fn new() -> FullMove {
        FullMove {
            white_move: None,
            white_seconds: 0.0,
            black_move: None,
            black_seconds: 0.0,
        }
    }

    pub fn get_white_move(&self) -> &Option<HalfMove> {
        &self.white_move
    }

    pub fn get_white_seconds(&self) -> f32 {
        self.white_seconds
    }

    pub fn get_black_move(&self) -> &Option<HalfMove> {
        &self.black_move
    }

    pub fn get_black_seconds(&self) -> f32 {
        self.black_seconds
    }

    pub fn set_white_move(&mut self, m: HalfMove) -> () {
        self.white_move = Some(m);
    }

    pub fn set_white_seconds(&mut self, seconds: f32) -> () {
        self.white_seconds = seconds;
    }

    pub fn set_black_move(&mut self, m: HalfMove) -> () {
        self.black_move = Some(m);
    }

    pub fn set_black_seconds(&mut self, seconds: f32) -> () {
        self.black_seconds = seconds;
    }
}

impl Display for FullMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let white_move = self
            .get_white_move()
            .as_ref()
            .map_or(String::from("..."), |half_move| format!("{}", half_move));
        let black_move = self
            .get_black_move()
            .as_ref()
            .map_or(String::from("..."), |half_move| format!("{}", half_move));

        let white_value = self.get_white_move().map_or(0.0, |v| v.evaluation);
        let black_value = self.get_black_move().map_or(0.0, |v| v.evaluation);

        f.write_str(&format!(
            "{} {} ({:010.6} - {:010.6}) [{:07.4} - {:07.4}]",
            white_move,
            black_move,
            self.white_seconds,
            self.black_seconds,
            white_value,
            black_value
        ))
    }
}
