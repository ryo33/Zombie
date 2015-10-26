extern crate piston_window;
extern crate graphics;
extern crate sprite;

use game::GameSettings;

mod game;

fn main() {
    let mut game = GameSettings::new().get_game();
    game.run();
}
