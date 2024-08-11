use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::model::board::board::Board;
use crate::model::board::square::Square;
use crate::model::piece::bishop::Bishop;
use crate::model::piece::color::Colour;
use crate::model::piece::king::King;
use crate::model::piece::knight::Knight;
use crate::model::piece::pawn::Pawn;
use crate::model::piece::piece::Piece;
use crate::model::piece::queen::Queen;
use crate::model::piece::rook::Rook;

pub static LOSS: f32 = f32::MIN;
pub static DRAW: f32 = f32::MIN + 1f32;
pub static WIN: f32 = f32::MAX;
pub static MAX_CONSECUTIVE_MOVES: u8 = 100;
pub static MATERIAL_FACTOR: f32 = 1.0;
pub static ACTIVITY_FACTOR: f32 = 1.0;
pub static PROXIMITY_FACTOR: f32 = 1.0;

pub fn normalize_rank(rank: u8) -> u8 {
    rank + 1
}

pub fn normalize_file(file: u8) -> char {
    (97 + file) as char
}

pub fn hash_position(board: &Board, colour: Colour) -> u64 {
    let mut state = DefaultHasher::new();
    board.hash(&mut state);
    colour.hash(&mut state);

    state.finish()
}

pub fn init_squares() -> [[Square; 8]; 8] {
    let rank_0: [Square; 8] = [
        Square::new(0, 0),
        Square::new(0, 1),
        Square::new(0, 2),
        Square::new(0, 3),
        Square::new(0, 4),
        Square::new(0, 5),
        Square::new(0, 6),
        Square::new(0, 7),
    ];
    let rank_1: [Square; 8] = [
        Square::new(1, 0),
        Square::new(1, 1),
        Square::new(1, 2),
        Square::new(1, 3),
        Square::new(1, 4),
        Square::new(1, 5),
        Square::new(1, 6),
        Square::new(1, 7),
    ];
    let rank_2: [Square; 8] = [
        Square::new(2, 0),
        Square::new(2, 1),
        Square::new(2, 2),
        Square::new(2, 3),
        Square::new(2, 4),
        Square::new(2, 5),
        Square::new(2, 6),
        Square::new(2, 7),
    ];
    let rank_3: [Square; 8] = [
        Square::new(3, 0),
        Square::new(3, 1),
        Square::new(3, 2),
        Square::new(3, 3),
        Square::new(3, 4),
        Square::new(3, 5),
        Square::new(3, 6),
        Square::new(3, 7),
    ];
    let rank_4: [Square; 8] = [
        Square::new(4, 0),
        Square::new(4, 1),
        Square::new(4, 2),
        Square::new(4, 3),
        Square::new(4, 4),
        Square::new(4, 5),
        Square::new(4, 6),
        Square::new(4, 7),
    ];
    let rank_5: [Square; 8] = [
        Square::new(5, 0),
        Square::new(5, 1),
        Square::new(5, 2),
        Square::new(5, 3),
        Square::new(5, 4),
        Square::new(5, 5),
        Square::new(5, 6),
        Square::new(5, 7),
    ];
    let rank_6: [Square; 8] = [
        Square::new(6, 0),
        Square::new(6, 1),
        Square::new(6, 2),
        Square::new(6, 3),
        Square::new(6, 4),
        Square::new(6, 5),
        Square::new(6, 6),
        Square::new(6, 7),
    ];
    let rank_7: [Square; 8] = [
        Square::new(7, 0),
        Square::new(7, 1),
        Square::new(7, 2),
        Square::new(7, 3),
        Square::new(7, 4),
        Square::new(7, 5),
        Square::new(7, 6),
        Square::new(7, 7),
    ];
    [
        rank_0, rank_1, rank_2, rank_3, rank_4, rank_5, rank_6, rank_7,
    ]
}

pub fn init_white_pieces() -> HashMap<Square, Box<dyn Piece>> {
    let mut pieces: HashMap<Square, Box<dyn Piece>> = HashMap::new();
    pieces.insert(Square::new(0, 0), Box::new(Rook::new(Colour::White)));
    pieces.insert(Square::new(0, 1), Box::new(Knight::new(Colour::White)));
    pieces.insert(Square::new(0, 2), Box::new(Bishop::new(Colour::White)));
    pieces.insert(Square::new(0, 3), Box::new(Queen::new(Colour::White)));
    pieces.insert(Square::new(0, 4), Box::new(King::new(Colour::White)));
    pieces.insert(Square::new(0, 5), Box::new(Bishop::new(Colour::White)));
    pieces.insert(Square::new(0, 6), Box::new(Knight::new(Colour::White)));
    pieces.insert(Square::new(0, 7), Box::new(Rook::new(Colour::White)));
    pieces.insert(Square::new(1, 0), Box::new(Pawn::new(Colour::White)));
    pieces.insert(Square::new(1, 1), Box::new(Pawn::new(Colour::White)));
    pieces.insert(Square::new(1, 2), Box::new(Pawn::new(Colour::White)));
    pieces.insert(Square::new(1, 3), Box::new(Pawn::new(Colour::White)));
    pieces.insert(Square::new(1, 4), Box::new(Pawn::new(Colour::White)));
    pieces.insert(Square::new(1, 5), Box::new(Pawn::new(Colour::White)));
    pieces.insert(Square::new(1, 6), Box::new(Pawn::new(Colour::White)));
    pieces.insert(Square::new(1, 7), Box::new(Pawn::new(Colour::White)));

    pieces
}
// pub fn init_white_pieces() -> HashMap<Square, Box<dyn Piece>> {
//     let mut pieces: HashMap<Square, Box<dyn Piece>> = HashMap::new();
//     pieces.insert(Square::new(6, 7), Box::new(Rook::new(Colour::White)));
//     pieces.insert(Square::new(6, 6), Box::new(Pawn::new(Colour::White)));
//     pieces.insert(Square::new(5, 6), Box::new(Pawn::new(Colour::White)));
//     pieces.insert(Square::new(4, 6), Box::new(Pawn::new(Colour::White)));
//     pieces.insert(Square::new(4, 7), Box::new(Pawn::new(Colour::White)));
//     pieces.insert(Square::new(5, 7), Box::new(King::new(Colour::White)));
//     pieces.insert(Square::new(7, 7), Box::new(Bishop::new(Colour::White)));
//
//     pieces
// }

pub fn init_black_pieces() -> HashMap<Square, Box<dyn Piece>> {
    let mut pieces: HashMap<Square, Box<dyn Piece>> = HashMap::new();
    pieces.insert(Square::new(7, 0), Box::new(Rook::new(Colour::Black)));
    pieces.insert(Square::new(7, 1), Box::new(Knight::new(Colour::Black)));
    pieces.insert(Square::new(7, 2), Box::new(Bishop::new(Colour::Black)));
    pieces.insert(Square::new(7, 3), Box::new(Queen::new(Colour::Black)));
    pieces.insert(Square::new(7, 4), Box::new(King::new(Colour::Black)));
    pieces.insert(Square::new(7, 5), Box::new(Bishop::new(Colour::Black)));
    pieces.insert(Square::new(7, 6), Box::new(Knight::new(Colour::Black)));
    pieces.insert(Square::new(7, 7), Box::new(Rook::new(Colour::Black)));
    pieces.insert(Square::new(6, 0), Box::new(Pawn::new(Colour::Black)));
    pieces.insert(Square::new(6, 1), Box::new(Pawn::new(Colour::Black)));
    pieces.insert(Square::new(6, 2), Box::new(Pawn::new(Colour::Black)));
    pieces.insert(Square::new(6, 3), Box::new(Pawn::new(Colour::Black)));
    pieces.insert(Square::new(6, 4), Box::new(Pawn::new(Colour::Black)));
    pieces.insert(Square::new(6, 5), Box::new(Pawn::new(Colour::Black)));
    pieces.insert(Square::new(6, 6), Box::new(Pawn::new(Colour::Black)));
    pieces.insert(Square::new(6, 7), Box::new(Pawn::new(Colour::Black)));

    pieces
}
// pub fn init_black_pieces() -> HashMap<Square, Box<dyn Piece>> {
//     let mut pieces: HashMap<Square, Box<dyn Piece>> = HashMap::new();
//     pieces.insert(Square::new(6, 4), Box::new(King::new(Colour::Black)));
//
//     pieces
// }
