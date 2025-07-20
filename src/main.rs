mod game;
use game::*;
mod settings;
use settings::*;
mod entities;
use entities::*;
fn main() {
    let mut game = Game::new();
    game.run();
}
