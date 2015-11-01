use std::collections::HashMap;
use super::chunk::Chunk;
use ::game::player::Player;
use graphics::{ Image, Graphics, ImageSize };
use graphics::Transformed;
use graphics::math::{ Scalar, Vec2d, Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };
use ::util::graphics;
use ::game::Context;
use ::game::camera::Camera;

pub struct World {
    player: i32,
    chunks: HashMap<(i32, i32), Chunk>,
    camera: Camera,
}

impl World {
    pub fn new() -> World {
        let mut chunks = HashMap::new();
        chunks.insert((0, 0), Chunk::new());
        World {
            player: 1,
            chunks: chunks,
            camera: Camera::new(),
        }
    }

    pub fn draw(&self, con: &Context, t: Matrix2d, b: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        // camera
        let t = con.trans(t).append_transform(self.camera.get_transform());
        // /camera
        let chunk = self.chunks.get(&(0, 0)).expect("Could not load chunk");
        for (x, y, _tile) in chunk.iter() {
            graphics::Image::new(con.get_tile("rust"), [0., 0., 10., 10.]).draw(b, t.trans(x as Scalar * 10., y as Scalar * 10.));
        }
    }
}
