use piston_window::*;

use game::InputManager;
use game::Game;

pub struct GameLoop {
    width: u32,
    height: u32,
    window: PistonWindow,
}

impl GameLoop {
    pub fn new(s: &GameLoopSettings) -> GameLoop {
        let opengl = OpenGL::V4_5;
        let mut window: PistonWindow =
            WindowSettings::new(s.title, (s.window_width, s.window_height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();
        //window.set_capture_cursor(true);
        GameLoop {
            width: s.width,
            height: s.height,
            window: window,
        }
    }

    pub fn run(&mut self, input_manager: &InputManager) {
        let mut game = Game::new(&mut *self.window.factory.borrow_mut(), self.width, self.height);
        for e in self.window.clone() {
            e.draw_2d(|c, g| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                let size = e.size();
                let width_ratio = size.width as f64 / self.width as f64;
                let height_ratio = size.height as f64 / self.height as f64;
                let mut con: Context;
                if width_ratio < height_ratio {
                    con = c.trans(0.0, (size.height as f64 - self.height as f64 * width_ratio) / 2.0).scale(width_ratio, width_ratio)
                } else {
                    con = c.trans((size.width as f64 - self.width as f64 * height_ratio) / 2.0, 0.0).scale(height_ratio, height_ratio)
                }
                game.draw(con.transform, g);
            });
            e.mouse_cursor(|x, y| {
                game.cursor(x, y);
            });
            e.update(|args| {
                game.update(args.dt);
            });
            e.press(|button| {
                game.press(input_manager.get_operation(button));
            });
            e.release(|button| {
                game.release(input_manager.get_operation(button));
            });
        }
    }
}

pub struct GameLoopSettings {
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
    title: &'static str,
}

#[allow(dead_code)]
impl GameLoopSettings {
    pub fn new() -> GameLoopSettings {
        GameLoopSettings {
            width: 1920,
            height: 1080,
            window_width: 960,
            window_height: 540,
            title: "game",
        }
    }

    pub fn default() -> GameLoop {
        GameLoopSettings::new().get_game()
    }

    pub fn internal_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn get_game(&self) -> GameLoop {
        GameLoop::new(self)
    }
}
