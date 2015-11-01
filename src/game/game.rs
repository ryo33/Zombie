use sprite::*;
use graphics::Transformed;
use graphics::math::{ Scalar, Vec2d, Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };
use gfx_texture::{ Texture };
use ::util::texture_register::TextureRegister;
use gfx;
use std::rc::Rc;

use game::world::World;

pub struct Game {
    world: World,
    context: Context,
}

impl Game {
    pub fn new<F: gfx::Factory<Resources>>(factory: &mut F, width: u32, height: u32) -> Game {
        Game {
            world: World::new(),
            context: Context::new(factory, width, height),
        }
    }

    pub fn draw(&self, t: Matrix2d, b: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        self.world.draw(&self.context, t, b);
    }

    pub fn update(&mut self) {
    }
}

pub struct Context {
    tile_textures: TextureRegister<Resources>,
    width: u32,
    height: u32,
}

impl Context {
    pub fn new<F: gfx::Factory<Resources>>(factory: &mut F, width: u32, height: u32) -> Context {
        Context {
            tile_textures: TextureRegister::new("tiles", factory),
            width: width,
            height: height,
        }
    }

    pub fn get_tile(&self, name: &str) -> Rc<Texture<Resources>> {
        self.tile_textures.get(name)
    }

    pub fn trans(&self, t: Matrix2d) -> Matrix2d {
        t.trans((self.width / 2) as Scalar, (self.height / 2) as Scalar)
    }
}
