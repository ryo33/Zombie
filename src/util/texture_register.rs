use std::collections::{ HashMap };
use std::rc::Rc;
use gfx_texture::{ Texture, TextureSettings, Flip };
use gfx;
use std::fs;
use find_folder::Search;

pub struct TextureRegister<R> where R: gfx::Resources {
    textures: HashMap<String, Rc<Texture<R>>>,
}

impl<R> TextureRegister<R> where R: gfx::Resources {
    pub fn new<F: gfx::Factory<R>>(dir: &str, factory: &mut F) -> TextureRegister<R> {
        let mut folder = Search::ParentsThenKids(3, 3).for_folder("assets").ok().expect("Could not find assets directory");
        folder.push(dir);
        let paths = fs::read_dir(folder).ok().expect("Could not find textures");
        let mut map = HashMap::new();
        for path in paths {
            let path = path.unwrap();
            if let Some(extension) = path.path().extension() {
                if extension == "png" {
                    let texture = Texture::from_path(factory, path.path(), Flip::None, &TextureSettings::new()).ok()
                        .expect(&format!("Could not load texture: {}", path.path().display()));
                    map.insert(path.path().file_stem().unwrap().to_str().unwrap().to_string(), Rc::new(texture));
                }
            }
        }
        TextureRegister {
            textures: map,
        }
    }

    pub fn get(&self, name: &str) -> Rc<Texture<R>> {
        self.textures.get(&name.to_string()).expect(&format!("Could not find texture: {}", name)).clone()
    }
}
