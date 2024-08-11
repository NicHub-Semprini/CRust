use crate::game::game::Game;

mod game;
mod model;
mod utils;

fn main() {
    let mut game = Game::new();
    println!("{}", game);

    game.play(2);
    println!("{}", game);
}
