use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use colored::{Colorize, CustomColor};

use crate::game::play::HalfMove;
use crate::model::board::square::Square;
use crate::model::piece::color::Colour;
use crate::model::piece::king::King;
use crate::model::piece::pawn::Pawn;
use crate::model::piece::piece::{Piece, PieceType};
use crate::model::piece::queen::Queen;

#[derive(Debug)]
pub struct Board {
    squares: [[Square; 8]; 8],
    white_pieces: HashMap<Square, Box<dyn Piece>>,
    black_pieces: HashMap<Square, Box<dyn Piece>>,
}

impl Board {
    pub fn new(
        squares: [[Square; 8]; 8],
        white_pieces: HashMap<Square, Box<dyn Piece>>,
        black_pieces: HashMap<Square, Box<dyn Piece>>,
    ) -> Board {
        Board {
            squares,
            white_pieces,
            black_pieces,
        }
    }

    pub fn duplicate(&self) -> Board {
        let squares = self.squares.clone();
        let mut white_pieces = HashMap::new();
        for (s, wp) in self.get_white_pieces() {
            white_pieces.insert(s.clone(), wp.duplicate());
        }
        let mut black_pieces = HashMap::new();
        for (s, bp) in self.get_black_pieces() {
            black_pieces.insert(s.clone(), bp.duplicate());
        }

        Board::new(squares, white_pieces, black_pieces)
    }

    pub fn is_square_free(&self, square: &Square) -> bool {
        self.white_pieces.get(square).is_none() && self.black_pieces.get(square).is_none()
    }

    fn is_on_board(rank: i8, file: i8) -> bool {
        rank >= 0 && rank < 8 && file >= 0 && file < 8
    }

    pub fn get_square(&self, rank: i8, file: i8) -> Option<&Square> {
        if Board::is_on_board(rank, file) {
            let rank = rank as usize;
            let file = file as usize;
            Some(&self.squares[rank][file])
        } else {
            None
        }
    }

    pub fn get_piece(&self, square: Square, colour: Option<Colour>) -> Option<Box<dyn Piece>> {
        let piece = match colour {
            Some(c) => match c {
                Colour::White => self.white_pieces.get(&square),
                Colour::Black => self.black_pieces.get(&square),
            },
            None => self
                .white_pieces
                .get(&square)
                .or(self.black_pieces.get(&square)),
        };
        match piece {
            Some(p) => Some(p.duplicate()),
            None => None,
        }
    }

    pub fn get_squares(&self) -> Vec<Square> {
        let mut squares = vec![];

        for rank in self.squares {
            for square in rank {
                squares.push(square)
            }
        }

        squares
    }

    fn get_white_pieces(&self) -> &HashMap<Square, Box<dyn Piece>> {
        &self.white_pieces
    }

    fn get_black_pieces(&self) -> &HashMap<Square, Box<dyn Piece>> {
        &self.black_pieces
    }

    pub fn get_pieces(&self, colour: Colour) -> &HashMap<Square, Box<dyn Piece>> {
        match colour {
            Colour::White => self.get_white_pieces(),
            Colour::Black => self.get_black_pieces(),
        }
    }

    pub fn execute_move(&mut self, play: &HalfMove) -> () {
        let from_square = play.get_from();
        let to_square = play.get_to();
        let colour = play.get_piece().get_colour();
        let mut piece = self
            .get_piece(from_square, Some(colour))
            .unwrap()
            .duplicate();

        // Piece has been moved
        piece.set_first_move(false);

        // Execute move for player
        match colour {
            Colour::White => self.execute_white_move(
                piece,
                from_square,
                to_square,
                play.is_capture(),
                play.is_promotion(),
            ),
            Colour::Black => self.execute_black_move(
                piece,
                from_square,
                to_square,
                play.is_capture(),
                play.is_promotion(),
            ),
        }
    }

    fn execute_white_move(
        &mut self,
        piece: Box<dyn Piece>,
        from: Square,
        to: Square,
        is_capture: bool,
        is_promotion: bool,
    ) -> () {
        // Eventually remove black piece sitting in target square
        if is_capture {
            self.black_pieces.remove(&to);
        }

        // Move white piece
        self.white_pieces.remove(&from);

        // Eventually promote
        if is_promotion {
            let mut promoted_piece = Box::new(PieceType::Queen(Queen::new(Colour::White)));
            promoted_piece.set_first_move(false);
            self.white_pieces.insert(to, promoted_piece);
        } else {
            self.white_pieces.insert(to, piece);
        }
    }

    fn execute_black_move(
        &mut self,
        piece: Box<dyn Piece>,
        from: Square,
        to: Square,
        is_capture: bool,
        is_promotion: bool,
    ) -> () {
        // Eventually remove white piece sitting in target square
        if is_capture {
            self.white_pieces.remove(&to);
        }

        // Move white piece
        self.black_pieces.remove(&from);

        // Eventually promote
        if is_promotion {
            let mut promoted_piece = Box::new(PieceType::Queen(Queen::new(Colour::Black)));
            promoted_piece.set_first_move(false);
            self.black_pieces.insert(to, promoted_piece);
        } else {
            self.black_pieces.insert(to, piece);
        }
    }

    pub fn is_promotion_square(&self, square: &Square, colour: Colour) -> bool {
        match colour {
            Colour::White => square.get_rank() == 7,
            Colour::Black => square.get_rank() == 0,
        }
    }

    pub fn is_check(&self, play: HalfMove) -> bool {
        let mut board = self.duplicate();
        board.execute_move(&play);

        board.is_under_check(play.get_piece().get_colour().get_opposite())
    }

    pub fn is_under_check(&self, colour: Colour) -> bool {
        let mut king_square = None;
        for (square, piece) in self.get_pieces(colour) {
            if piece.get_type() == PieceType::King(King::new(colour)) {
                king_square = Some(square);
                break;
            }
        }

        let king_square = king_square.expect(&format!("Cannot find {} king", colour));

        self.is_under_control(king_square, colour.get_opposite())
    }

    fn is_under_control(&self, square: &Square, colour: Colour) -> bool {
        self.get_pieces(colour)
            .iter()
            .any(|(s, p)| p.is_controlling(self, s, square))
    }

    pub fn evaluate_material(&self, colour: Colour) -> f32 {
        let allie_material = self
            .get_pieces(colour)
            .iter()
            .map(|(_, p)| p.get_value() as f32)
            .reduce(|v1, v2| v1 + v2)
            .unwrap();
        let enemy_material = self
            .get_pieces(colour.get_opposite())
            .iter()
            .map(|(_, p)| p.get_value() as f32)
            .reduce(|v1, v2| v1 + v2)
            .unwrap();

        allie_material / enemy_material
    }

    pub fn evaluate_activity(&self, colour: Colour) -> f32 {
        let squares = self.get_squares();

        let allie_activity = squares
            .iter()
            .map(|s| self.is_under_control(s, colour))
            .filter(|under_control| *under_control == true)
            .count() as f32;
        let enemy_activity = squares
            .iter()
            .map(|s| self.is_under_control(s, colour.get_opposite()))
            .filter(|under_control| *under_control == true)
            .count() as f32;

        allie_activity / enemy_activity
    }

    pub fn evaluate_proximity(&self, colour: Colour) -> f32 {
        let pawns_proximity = self
            .get_pieces(colour)
            .iter()
            .filter(|(_, piece)| piece.get_type() == PieceType::Pawn(Pawn::new(colour)))
            .map(|(square, _)| square.get_rank() as f32)
            .map(|rank| match colour {
                Colour::White => rank,
                Colour::Black => 7.0 - rank,
            })
            .map(|v| 1.0 - (1.0 / v))
            .reduce(|v1, v2| v1 + v2)
            .unwrap_or(0.0);

        pawns_proximity
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.squares.hash(state);

        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(rank, file);
                match self.white_pieces.get(&square) {
                    Some(piece) => {
                        square.hash(state);
                        piece.get_type().hash(state);
                    }
                    None => {}
                }
                match self.black_pieces.get(&square) {
                    Some(piece) => {
                        square.hash(state);
                        piece.get_type().hash(state);
                    }
                    None => {}
                }
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(" ")?;
        for a in 'a'..='h' {
            f.write_str(format!("   {}", a).as_str())?;
        }
        f.write_str("\n")?;
        for i in (0..=7).rev() {
            f.write_str(format!("{} |", i + 1).as_str())?;
            for y in 0..=7 {
                let square = self.get_square(i, y).unwrap();
                let piece = match self.white_pieces.get(square) {
                    Some(p) => &format!(" {} ", p),
                    None => match self.black_pieces.get(square) {
                        Some(p) => &format!(" {} ", p),
                        None => "   ",
                    },
                };
                let is_even = (square.get_rank() + square.get_file()) % 2 == 0;
                if is_even {
                    f.write_str(&format!(
                        "{}",
                        piece.on_custom_color(CustomColor::new(206, 206, 206))
                    ))?;
                } else {
                    f.write_str(&format!("{}", piece))?;
                }
                f.write_str("|")?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
