use gfx_graphics::{ GfxGraphics };
use gfx_device_gl::{ Resources, CommandBuffer, Output };

pub type Graphics<'a> = GfxGraphics<'a, Resources, CommandBuffer<Resources>, Output>;
