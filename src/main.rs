extern crate sdl2;
mod game;

use game::game::Game;

fn main() {
    let mut game = Game::new("rSnake".to_string());
    game.get_title();
    game.game_loop();
}
