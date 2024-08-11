use std::fmt::{Display, Formatter};

use anyhow::Error;

use crate::game::clock::{Clock, OpenClock};
use crate::game::play::{FullMove, HalfMove};
use crate::game::turn::Turn;
use crate::model::board::board::Board;
use crate::model::piece::color::Colour;
use crate::model::piece::pawn::Pawn;
use crate::model::piece::piece::{Piece, PieceType};
use crate::utils;

pub struct Game {
    turns: Vec<Turn>,
    board: Board,
    consecutive_moves: u8,
    positions: Vec<u64>,
    white_clock: Clock,
    black_clock: Clock,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.board.fmt(f)?;
        for turn in &self.turns {
            f.write_str("\n")?;
            turn.fmt(f)?;
        }
        let time =
            match self.white_clock.get_total_seconds() >= self.black_clock.get_total_seconds() {
                true => self.white_clock.get_total_seconds(),
                false => self.black_clock.get_total_seconds(),
            };
        f.write_str("\n\n")?;
        f.write_str(&format!("{}s", time))?;
        Ok(())
    }
}

impl Game {
    pub fn new() -> Game {
        let squares = utils::init_squares();
        let white_pieces = utils::init_white_pieces();
        let black_pieces = utils::init_black_pieces();
        let board = Board::new(squares, white_pieces, black_pieces);
        let hash = utils::hash_position(&board, Colour::White);

        Game {
            turns: vec![],
            board,
            consecutive_moves: 0,
            positions: vec![hash],
            white_clock: Clock::new(),
            black_clock: Clock::new(),
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_turns(&self) -> &Vec<Turn> {
        &self.turns
    }

    fn open_clock(&self, colour: Colour) -> Clock<OpenClock> {
        match colour {
            Colour::White => self.white_clock.open(),
            Colour::Black => self.black_clock.open(),
        }
    }

    fn close_clock(&mut self, colour: Colour, clock: Clock<OpenClock>) -> () {
        match colour {
            Colour::White => self.white_clock = clock.close(),
            Colour::Black => self.black_clock = clock.close(),
        }
    }

    pub fn play(&mut self, max_depth: u8) -> () {
        let mut count = 0;
        let mut white_move = None;
        let mut black_move = None;
        loop {
            match self.is_over(count, white_move, black_move) {
                Ok(_) => {
                    count = count + 1;
                    let mut full_move = FullMove::new();

                    // Play as White
                    white_move = self.play_turn(Colour::White, max_depth);
                    if white_move.is_none() {
                        black_move = self.eventually_set_checkmate(black_move);
                        black_move = self.eventually_set_stalemate(black_move);
                        let turn = self.turns.pop().unwrap();
                        let mut full_move = FullMove::new();
                        full_move.set_white_move(turn.get_moves().get_white_move().unwrap());
                        full_move.set_black_move(black_move.unwrap());
                        self.turns.push(Turn::new(turn.get_index(), full_move));
                        continue;
                    }
                    full_move.set_white_move(white_move.unwrap());
                    full_move.set_white_seconds(self.white_clock.get_last_seconds());

                    // Play as Black
                    black_move = self.play_turn(Colour::Black, max_depth);
                    if black_move.is_none() {
                        white_move = self.eventually_set_checkmate(white_move);
                        white_move = self.eventually_set_stalemate(white_move);
                        full_move.set_white_move(white_move.unwrap());
                        self.turns.push(Turn::new(count, full_move));
                        continue;
                    }
                    full_move.set_black_move(black_move.unwrap());
                    full_move.set_black_seconds(self.black_clock.get_last_seconds());

                    let turn = Turn::new(count, full_move);
                    println!("{}", turn);
                    println!("{}", self.board);
                    self.turns.push(turn);
                }
                Err(error) => {
                    println!("Game over. Reason: {}", error);
                    break;
                }
            }
        }
    }

    pub fn play_turn(&mut self, colour: Colour, max_depth: u8) -> Option<HalfMove> {
        // Start timer
        let clock = self.open_clock(colour);

        let moves = self.select_legal_moves(&self.board, colour);

        // Choose best move
        if !moves.is_empty() {
            let chosen_one = self
                .choose_move(
                    &self.board.duplicate(),
                    colour,
                    colour,
                    moves,
                    self.consecutive_moves,
                    &self.positions.clone(),
                    0,
                    max_depth,
                )
                .0
                .expect("There are no moves to play");
            self.board.execute_move(&chosen_one);

            // Stop timer
            self.close_clock(colour, clock);

            // Keep track of how many consecutive moves have been done without moving a pawn or capturing
            self.consecutive_moves =
                Game::update_consecutive_moves(&chosen_one, self.consecutive_moves);

            // Keep track of played positions
            self.positions
                .push(utils::hash_position(&self.board, colour.get_opposite()));

            return Some(chosen_one);
        }

        // End of the game
        None
    }

    fn update_consecutive_moves(play: &HalfMove, consecutive_moves: u8) -> u8 {
        if play.is_capture() || play.get_piece() == PieceType::Pawn(Pawn::new(Colour::White)) {
            0
        } else {
            consecutive_moves + 1
        }
    }

    fn select_legal_moves(&self, board: &Board, colour: Colour) -> Vec<HalfMove> {
        let mut moves = vec![];
        for (square, piece) in board.get_pieces(colour) {
            for m in piece.available_moves(board, square) {
                moves.push(m);
            }
        }

        // Keep only legal moves
        let moves = self.remove_forbidden_moves(board, moves, colour);

        // Check for checking moves
        let mut final_moves = vec![];
        for mut m in moves {
            m.set_check(board.is_check(m));
            final_moves.push(m);
        }

        final_moves
    }

    fn remove_forbidden_moves(
        &self,
        board: &Board,
        moves: Vec<HalfMove>,
        colour: Colour,
    ) -> Vec<HalfMove> {
        let mut valid_moves = vec![];
        for m in moves {
            let mut board = board.duplicate();
            board.execute_move(&m);
            if !board.is_under_check(colour) {
                valid_moves.push(m);
            }
        }

        valid_moves
    }

    fn eventually_set_checkmate(&self, play: Option<HalfMove>) -> Option<HalfMove> {
        match play {
            Some(mut p) => {
                p.set_checkmate(p.is_check());
                Some(p)
            }
            None => None,
        }
    }

    fn eventually_set_stalemate(&self, play: Option<HalfMove>) -> Option<HalfMove> {
        match play {
            Some(mut p) => {
                p.set_stalemate(!p.is_checkmate());
                Some(p)
            }
            None => None,
        }
    }

    fn choose_move(
        &mut self,
        board: &Board,
        original_colour: Colour,
        colour: Colour,
        moves: Vec<HalfMove>,
        consecutive_moves: u8,
        positions: &Vec<u64>,
        depth: u8,
        max_depth: u8,
    ) -> (Option<HalfMove>, f32) {
        // Keep track of best move
        let mut best: (Option<HalfMove>, f32) = (None, utils::LOSS);

        // If there is only 1 move, just play it
        if moves.iter().count() == 1 {
            best.0 = Some(moves[0]);
            return best;
        }

        let is_king_alone = board.get_pieces(original_colour).iter().count() == 1;

        // Try all moves
        for m in moves {
            // The final position that will be evaluated
            let mut final_board = board.duplicate();
            final_board.execute_move(&m);
            let updated_consecutive_moves = Game::update_consecutive_moves(&m, consecutive_moves);
            let mut actual = (Some(m), utils::LOSS);
            let mut positions = positions.clone();
            positions.push(utils::hash_position(&final_board, colour.get_opposite()));

            if Game::is_max_consecutive_moves(consecutive_moves) {
                // Base case: draw for 50 consecutive moves
                actual.1 = Game::compute_draw_value(is_king_alone);
            } else if Game::is_triple_repetition(&final_board, colour.get_opposite(), &positions) {
                // Base case: draw for triple repetition
                actual.1 = Game::compute_draw_value(is_king_alone);
            } else if depth >= max_depth {
                // Base case: stop recursion and evaluate final position
                actual.1 = self.evaluate(&final_board, original_colour);
            } else {
                // Play as enemy
                let new_colour = colour.get_opposite();
                let new_moves = self.select_legal_moves(&final_board, new_colour);

                // Base case: game ended
                if new_moves.is_empty() {
                    if !m.is_check() {
                        // Stalemate
                        actual.1 = Game::compute_draw_value(is_king_alone);
                    } else if m.get_piece().get_colour() == original_colour {
                        // Checkmate for enemy
                        actual.1 = utils::WIN;
                    } else {
                        // Checkmate for player
                        actual.1 = utils::LOSS;
                    }
                } else {
                    // Go to next play
                    actual = self.choose_move(
                        &final_board,
                        original_colour,
                        new_colour,
                        new_moves,
                        updated_consecutive_moves,
                        &positions,
                        depth + 1,
                        max_depth,
                    );
                }
            }

            // Select the move if is better than best one or if is the first one evaluated
            if actual.1 > best.1 || best.0.is_none() {
                best.1 = actual.1;
                let mut best_move = m;
                best_move.set_evaluation(actual.1);
                best.0 = Some(best_move);
            }

            // If there is a winning move, just play it
            if best.1 == utils::WIN {
                return best;
            }
        }

        best
    }

    fn compute_draw_value(is_king_alone: bool) -> f32 {
        // If only the king is on the board, draw is equal to win
        match is_king_alone {
            true => utils::WIN,
            false => utils::DRAW,
        }
    }

    fn evaluate(&self, board: &Board, colour: Colour) -> f32 {
        // Evaluate material
        let material = board.evaluate_material(colour);
        // Evaluate activity
        let activity = board.evaluate_activity(colour);
        // Evaluate proximity
        let proximity = board.evaluate_proximity(colour);

        // TODO other evaluations (king safety, ...)

        (material * utils::MATERIAL_FACTOR)
            + (activity * utils::ACTIVITY_FACTOR)
            + (proximity + utils::PROXIMITY_FACTOR)
    }

    fn is_over(
        &self,
        count: u8,
        last_white_move: Option<HalfMove>,
        last_black_move: Option<HalfMove>,
    ) -> Result<(), Error> {
        if count >= u8::MAX {
            Err(Error::msg("Max number of turns"))
        } else if Game::is_max_consecutive_moves(self.consecutive_moves) {
            Err(Error::msg(
                "50 consecutive turns without captures or pawn moves",
            ))
        } else if Game::is_triple_repetition(&self.board, Colour::White, &self.positions) {
            Err(Error::msg("Same position repeated for 3 times"))
        } else if !self.turns.is_empty() && last_white_move.is_none() {
            Err(Error::msg("White doesn't have any legal move to play"))
        } else if !self.turns.is_empty() && last_black_move.is_none() {
            Err(Error::msg("Black doesn't have any legal move to play"))
        } else {
            Ok(())
        }
    }

    fn is_triple_repetition(board: &Board, colour: Colour, positions: &Vec<u64>) -> bool {
        let hash = utils::hash_position(&board, colour);

        positions.iter().filter(|p| hash == **p).count() >= 3
    }

    fn is_max_consecutive_moves(consecutive_moves: u8) -> bool {
        // 100 moves (50 for each player)
        consecutive_moves >= utils::MAX_CONSECUTIVE_MOVES
    }
}
