use std::cell::{ RefCell, RefMut };
use std::rc::Rc;
use piston_window::G2d;
use graphics::Transformed;
use graphics::math::{ Scalar, Matrix2d };
use gfx_device_gl::{ Resources };
use gfx_texture::{ Texture };
use gfx;

use util::texture_register::TextureRegister;
use game::operation::*;
use game::world::World;
use game::input_state::InputState;
use game::{ UContext, DContext };

pub struct Game {
    pub width: u32,
    pub height: u32,
    pub world: World,
    pub tile_textures: RefCell<TextureRegister<Resources>>,
    pub input_state: RefCell<InputState>,
}

impl Game {
    pub fn new<F: gfx::Factory<Resources>>(factory: &mut F, width: u32, height: u32) -> Game {
        Game {
            width: width,
            height: height,
            world: World::new(width as Scalar, height as Scalar),
            tile_textures: RefCell::new(TextureRegister::new("tiles", factory)),
            input_state: RefCell::new(InputState::new()),
        }
    }

    pub fn draw(&mut self, t: Matrix2d, g: &mut G2d) {
        let mut con = DContext{
            width: self.width,
            height: self.height,
            transform: t,
            tile_textures: &mut *self.tile_textures.borrow_mut(),
        };
        self.world.draw(&mut con, g);
    }

    pub fn update(&mut self, dt: f64) {
        let mut con = UContext {
            width: self.width,
            height: self.height,
            input_state: &mut *self.input_state.borrow_mut(),
            dt: dt,
        };
        self.world.update(&mut con);
    }

    pub fn cursor(&mut self, x: f64, y: f64) {
        self.input_state.borrow_mut().cursor(x, y);
    }

    pub fn press(&mut self, op: Operation) {
        self.input_state.borrow_mut().press(op);
    }

    pub fn release(&mut self, op: Operation) {
        self.input_state.borrow_mut().release(op);
    }
}
