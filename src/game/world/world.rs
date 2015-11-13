use piston_window::G2d;
use std::collections::HashMap;
use graphics::Transformed;
use graphics::math::{ Scalar, Vec2d, Matrix2d };
use std::f64::consts::PI;

use super::chunk::Chunk;
use util::graphics::Image;
use game::camera::Camera;
use game::player::{ self, Player };
use game::*;
use game::operation::*;

pub const TILE_SIZE: f64 = 1.0;
pub const TILE_RECT: [f64; 4] = [0.0, 0.0, 1.0, 1.0];
const ROTATE_RATIO: f64 = 1.0;

pub struct World {
    player: Player,
    chunks: HashMap<(i32, i32), Chunk>,
    camera: Camera,
}

impl World {
    pub fn new(width: Scalar, height: Scalar) -> World {
        let mut chunks = HashMap::new();
        chunks.insert((0, 0), Chunk::new());
        World {
            player: Player::new(),
            chunks: chunks,
            camera: Camera::new(width, height),
        }
    }

    pub fn draw(&self, con: &mut DContext, g: &mut G2d) {
        con.transform = con.transform.append_transform(self.camera.get_transform());
        let chunk = self.chunks.get(&(0, 0)).expect("Could not load chunk");
        for (x, y, _tile) in chunk.iter() {
            Image::new(con.tile_textures.get("green"), TILE_RECT).draw(g, con.transform.trans(x as Scalar * TILE_SIZE, y as Scalar * TILE_SIZE));
        }
        self.player.draw(con, g);
    }

    pub fn update(&mut self, con: &mut UContext) {
        self.operation(con);
        self.camera.update(con, self.player.rotation);
        self.player.update(con, &mut self.camera);
    }

    fn operation(&mut self, con: &mut UContext) {
        for op in con.input_state.vec().iter() {
            match op {
                &Operation::Cursor(x, _y) => {
                    let rad = x * ROTATE_RATIO * con.dt;
                    self.player.rotate(rad);
                }
                _ => {},
            }
        }
    }
}
