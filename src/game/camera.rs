use graphics::math::{ Scalar, Vec2d, Matrix2d, identity };
use graphics::Transformed;
use std::f64::consts::PI;

use game::*;
use game::player;

const CAMERA_BASE: f64 = 0.8;

#[allow(dead_code)]
pub struct Camera {
    pub position: Vec2d,
    pub rotation: Scalar,
    pub scale: Scalar,
    trans_base: Matrix2d, // const
    bottom_corner_rad: f64, // const
}

impl Camera {
    pub fn new(width: Scalar, height: Scalar) -> Camera {
        Camera {
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: 100.0,
            trans_base: identity().trans(width as f64 / 2.0, height as f64 * CAMERA_BASE),
            bottom_corner_rad: ((height as f64 * (1.0 - CAMERA_BASE)) / (width as f64 / 2.0)).atan(),
        }
    }

    pub fn update(&mut self, con: &mut UContext, player_rotation: Scalar) {
        let limit_rad = PI / 2.0 + self.bottom_corner_rad - player::VIEW_RANGE / 2.0;
        if ! con.input_state.fix_camera {
            self.rotation = player_rotation;
        } else if (self.rotation - player_rotation).abs() > limit_rad {
            if self.rotation - player_rotation < 0.0 {
                self.rotation = player_rotation - limit_rad;
            } else {
                self.rotation = player_rotation + limit_rad;
            }
        }
    }

    pub fn get_transform(&self) -> Matrix2d {
        identity()
            .append_transform(self.trans_base)
            .rot_rad(- self.rotation)
            .scale(self.scale, self.scale)
            .trans(- self.position[0], - self.position[1])
    }

    pub fn move_by(&mut self, m: Vec2d) -> &mut Self {
        for i in 0..2 {
            self.position[i] = self.position[i] + m[i]
        }
        self
    }

    pub fn move_to(&mut self, m: Vec2d) -> &mut Self {
        for i in 0..2 {
            self.position[i] = m[i]
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

    pub fn scale(&mut self, s: Scalar) -> &mut Self {
        self.scale = self.scale * s;
        self
    }

    pub fn set_scale(&mut self, s: Scalar) -> &mut Self {
        self.scale = s;
        self
    }
}
