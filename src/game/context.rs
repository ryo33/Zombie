use std::cell::{ RefCell, RefMut };
use std::rc::Rc;
use graphics::math::{ Scalar, Matrix2d };
use gfx_device_gl::{ Resources };

use util::texture_register::TextureRegister;
use game::input_state::InputState;
use game::Game;

pub struct UpdateContext<'a> {
    pub width: u32,
    pub height: u32,
    pub input_state: &'a mut InputState,
    pub dt: Scalar,
}

pub struct DrawingContext<'a> {
    pub width: u32,
    pub height: u32,
    pub transform: Matrix2d,
    pub tile_textures: &'a TextureRegister<Resources>,
}
