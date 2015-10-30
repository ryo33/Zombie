use std::collections::{ HashMap };
use std::rc::Rc;
use graphics::{ ImageSize };

pub struct TextureRegister<ImageSize> {
    textures: HashMap<i32, ImageSize>,
}

impl<I: ImageSize> TextureRegister<I> {
    pub fn new() -> TextureRegister<I> {
        TextureRegister {
            textures: HashMap::new(),
        }
    }

    pub fn register(&mut self, num: i32, texture: I) -> &mut TextureRegister<I> {
        self.textures.insert(num, texture);
        self
    }

    pub fn get(& self, num: i32) -> Option<&I> {
        self.textures.get(&num)
    }
}
