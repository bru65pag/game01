extern crate sdl2;
mod game;

use game::game::Game;
use std::env;


fn main() {

    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing fps");
    };
    let fps = args[1].parse::<u64>().unwrap();
    println!("fps = {}", fps);

    let mut game = Game::new("rSnake".to_string(), fps);
    game.get_title();
    game.game_loop();
}
