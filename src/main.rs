extern crate piston_window;
extern crate graphics;
extern crate sprite;
extern crate find_folder;
extern crate gfx_graphics;
extern crate gfx_texture;
extern crate gfx;
extern crate gfx_device_gl;

use game_loop::GameLoopSettings;
use game::InputManager;

mod game_loop;
mod game;
mod util;
mod math;
mod types;

fn main() {
    let mut game_loop = GameLoopSettings::new().title("zombie").get_game();
    game_loop.run(&InputManager::new());
}
