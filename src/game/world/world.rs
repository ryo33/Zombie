use std::collections::HashMap;
use super::chunk::Chunk;
use ::game::player::Player;
use graphics::{ Graphics, ImageSize };
use graphics::math::{ Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };

pub struct World {
    player: i32,
    chunks: HashMap<(i32, i32), Chunk>,
}

impl World {
    pub fn new() -> World {
        let mut chunks = HashMap::new();
        chunks.insert((0, 0), Chunk::new());
        World {
            player: 1,
            chunks: chunks,
        }
    }

    pub fn draw(&self, t: Matrix2d, b: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        let a = self.chunks.get(&(0, 0));
    }
}
