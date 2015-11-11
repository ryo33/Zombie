use std::collections::HashMap;
use graphics::Transformed;
use graphics::math::{ Scalar, Vec2d, Matrix2d };
use std::f64::consts::PI;

use super::chunk::Chunk;
use util::graphics::Image;
use game::Context;
use game::camera::Camera;
use game::player::{ self, Player };
use game::operation::*;
use types::Graphics;

pub const TILE_SIZE: f64 = 1.0;
pub const TILE_RECT: [f64; 4] = [0.0, 0.0, 1.0, 1.0];
const ROTATE_RATIO: f64 = 1.0;

pub struct World {
    player: Player,
    chunks: HashMap<(i32, i32), Chunk>,
    camera: Camera,
}

impl World {
    pub fn new(con: &Context) -> World {
        let mut chunks = HashMap::new();
        chunks.insert((0, 0), Chunk::new());
        World {
            player: Player::new(),
            chunks: chunks,
            camera: Camera::new(con),
        }
    }

    pub fn draw(&self, con: &Context, t: Matrix2d, b: &mut Graphics) {
        // camera
        let t = t.append_transform(self.camera.get_transform());
        let chunk = self.chunks.get(&(0, 0)).expect("Could not load chunk");
        for (x, y, _tile) in chunk.iter() {
            Image::new(con.get_tile("green"), TILE_RECT).draw(b, t.trans(x as Scalar * TILE_SIZE, y as Scalar * TILE_SIZE));
        }
        self.player.draw(con, t, b);
    }

    pub fn update(&mut self, con: &Context, dt: f64) {
        self.camera.update(con.input_state.fix_camera, self.player.rotation);
    }

    pub fn operation(&mut self, op: &Operation, dt: f64) {
        let mut vec = [0.0, 0.0];
        match op {
            &Operation::Move(Direction::Left) => {
                vec = [- dt, 0.0];
            },
            &Operation::Move(Direction::Right) => {
                vec = [dt, 0.0];
            },
            &Operation::Move(Direction::Up) => {
                vec = [0.0, - dt];
            },
            &Operation::Move(Direction::Down) => {
                vec = [0.0, dt];
            },
            &Operation::Cursor(x, _y) => {
                let rad = x * ROTATE_RATIO * dt;
                self.player.rotate(rad);
            }
            _ => {},
        }
        let vec = self.player.move_dir(vec, &self.camera);
        self.camera.move_by(vec);
    }
}
