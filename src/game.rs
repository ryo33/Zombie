use piston_window::*;
use sprite::*;

pub struct Game {
    width: u32,
    height: u32,
    window: PistonWindow,
}

impl Game {
    pub fn new(width: u32, height: u32, window_width: u32, window_height: u32) -> Game {
        let opengl = OpenGL::V3_2;
        let window: PistonWindow =
            WindowSettings::new("piston: sprite", (window_width, window_height))
            .exit_on_esc(false)
            .opengl(opengl)
            .build()
            .unwrap();
        Game {
            width: width,
            height: height,
            window: window,
        }
    }

    pub fn run(&mut self) {
        for e in self.window.clone() {
            e.draw_2d(|c, g| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                let size = e.size();
                let width_ratio = size.width as f64 / self.width as f64;
                let height_ratio = size.height as f64 / self.height as f64;
                let con: Context;
                if width_ratio < height_ratio {
                    con = c.trans(0.0, (size.height as f64 - self.height as f64 * width_ratio) / 2.0).scale(width_ratio, width_ratio)
                } else {
                    con = c.trans((size.width as f64 - self.width as f64 * height_ratio) / 2.0, 0.0).scale(height_ratio, height_ratio)
                }
            });
        }
    }
}

pub struct GameSettings {
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
}

impl GameSettings {
    pub fn new() -> GameSettings {
        GameSettings {
            width: 1920,
            height: 1080,
            window_width: 960,
            window_height: 540,
        }
    }

    pub fn default() -> Game {
        GameSettings::new().get_game()
    }

    pub fn internal_size(&mut self, width: u32, height: u32) -> &mut GameSettings {
        self.width = width;
        self.height = height;
        self
    }

    pub fn window_size(&mut self, width: u32, height: u32) -> &mut GameSettings {
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn get_game(&self) -> Game {
        Game::new(self.width, self.height, self.window_width, self.window_height)
    }
}
