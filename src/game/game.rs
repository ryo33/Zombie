use piston_window::G2d;
use graphics::Transformed;
use graphics::math::{ Scalar, Matrix2d };
use gfx_device_gl::{ Resources };
use gfx_texture::{ Texture };
use gfx;
use std::rc::Rc;

use util::texture_register::TextureRegister;
use game::operation::*;
use game::world::World;
use game::input_state::InputState;

pub struct Game {
    world: World,
    context: Context,
}

impl Game {
    pub fn new<F: gfx::Factory<Resources>>(factory: &mut F, width: u32, height: u32) -> Game {
        let con = Context::new(factory, width, height, InputState::new());
        Game {
            world: World::new(&con),
            context: con,
        }
    }

    pub fn draw(&self, t: Matrix2d, b: &mut G2d) {
        self.world.draw(&self.context, t, b);
    }

    pub fn update(&mut self, dt: f64) {
        for op in self.context.input_state.vec().iter() {
            self.world.operation(op, dt);
        }
        self.world.update(&self.context, dt);
    }

    pub fn cursor(&mut self, x: f64, y: f64) {
        self.context.input_state.cursor(x, y);
    }

    pub fn press(&mut self, op: Operation) {
        self.context.input_state.press(op);
    }

    pub fn release(&mut self, op: Operation) {
        self.context.input_state.release(op);
    }
}

pub struct Context {
    tile_textures: TextureRegister<Resources>,
    pub width: u32,
    pub height: u32,
    pub input_state: InputState,
}

impl Context {
    pub fn new<F: gfx::Factory<Resources>>(factory: &mut F, width: u32, height: u32, input_state: InputState) -> Context {
        Context {
            tile_textures: TextureRegister::new("tiles", factory),
            width: width,
            height: height,
            input_state: input_state,
        }
    }

    pub fn get_tile(&self, name: &str) -> Rc<Texture<Resources>> {
        self.tile_textures.get(name)
    }
}
