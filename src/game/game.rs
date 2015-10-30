use sprite::*;
use graphics::math::{ Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };

use game::world::World;

pub struct Game {
    world: World
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: World::new(),
        }
    }

    pub fn draw(&self, t: Matrix2d, b: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        self.world.draw(t, b);
    }

    pub fn update(&mut self) -> &mut Game {
        self
    }
}
