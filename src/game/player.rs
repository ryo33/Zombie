use piston_window::G2d;
use graphics::math::{ Scalar, Vec2d, Matrix2d, identity };
use graphics::Transformed;
use std::f64::consts::PI;

use game::camera::Camera;
use game::world::TILE_RECT;
use game::world::TILE_SIZE;
use util::graphics::Image;
use game::*;

const CAMERA_BASE: f64 = 0.8;
const MOVE: f64 = 5.0;
pub const VIEW_RANGE: f64 = PI * 2f64 / 3f64; // 120 deg

#[allow(dead_code)]
pub struct Player {
    pub pos: Vec2d,
    pub rotation: Scalar,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: [0.0, 0.0],
            rotation: 0.0,
        }
    }

    pub fn get_transform(&self) -> Matrix2d {
        identity().rot_rad(self.rotation)
    }

    pub fn update(&mut self, con: &UContext, camera: &mut Camera) {
        let (mut x, mut y) = (0 as Scalar, 0 as Scalar);
        if con.input_state.left { x -= 1.0; }
        if con.input_state.right { x += 1.0; }
        if con.input_state.up { y -= 1.0; }
        if con.input_state.down { y += 1.0; }
        if x != 0.0 || y != 0.0 {
            let l = (x * x + y * y).sqrt();
            let vec = self.move_dir([x / l * con.dt, y / l * con.dt]);
            camera.move_by(vec);
        }
    }

    pub fn draw(&self, con: &mut DContext, g: &mut G2d) {
        Image::new(con.tile_textures.get("man"), TILE_RECT)
            .draw(g, con.transform
                  .trans(self.pos[0], self.pos[1])
                  .rot_rad(self.rotation)
                  .trans(- 0.5, - 0.5));
    }

    pub fn move_dir(&mut self, vec: [f64; 2]) -> Vec2d {
        use math::rotate_vec;
        let vec = rotate_vec([vec[0] * MOVE, vec[1] * MOVE], - self.rotation);
        self.move_by(vec);
        vec
    }

    pub fn move_by(&mut self, m: Vec2d) -> &mut Self {
        for i in 0..2 {
            self.pos[i] = self.pos[i] + m[i]
        }
        self
    }

    pub fn move_to(&mut self, m: Vec2d) -> &mut Self {
        for i in 0..2 {
            self.pos[i] = m[i]
        }
        self
    }

    pub fn rotate(&mut self, r: Scalar) -> &mut Self {
        self.rotation = self.rotation + r;
        self
    }

    pub fn set_rotation(&mut self, r: Scalar) -> &mut Self {
        self.rotation = r;
        self
    }
}
