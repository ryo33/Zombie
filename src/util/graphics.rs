use gfx_texture::{ Texture };
use std::rc::Rc;
use graphics;
use graphics::draw_state;
use graphics::math::{ Matrix2d };
use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };

pub struct Image {
    texture: Rc<Texture<Resources>>,
    rect: [f64; 4],
    color: [f32; 3],
    opacity: f32,
    draw_state: &'static draw_state::DrawState,
}

impl Image {
    pub fn new(texture: Rc<Texture<Resources>>, rect: [f64; 4]) -> Image {
        Image {
            texture: texture,
            rect: rect,
            color: [1f32, 1f32, 1f32],
            opacity: 1f32,
            draw_state: graphics::default_draw_state(),
        }
    }

    pub fn draw(&self, b: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, model: Matrix2d) -> &Image {
        graphics::Image::new()
            .color([self.color[0], self.color[1], self.color[2], self.opacity]).rect(self.rect)
            .draw(&*self.texture, self.draw_state, model, b);
        self
    }
}

