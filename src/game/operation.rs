use piston_window::{ Button, Key, MouseButton };

pub struct InputManager;

impl InputManager {
    pub fn new() -> InputManager {
        InputManager
    }

    pub fn get_operation(&self, button: Button) -> Operation {
        match button {
            Button::Keyboard(key) => match key {
                Key::A => {
                    Operation::Move(Direction::Left)
                },
                Key::D => {
                    Operation::Move(Direction::Right)
                },
                Key::W => {
                    Operation::Move(Direction::Up)
                },
                Key::S => {
                    Operation::Move(Direction::Down)
                },
                Key::LShift => {
                    Operation::FixCamera
                },
                _ => {Operation::None},
            },
            Button::Mouse(button) => match button {
                _ => {Operation::None},
            },
            _ => {Operation::None},
        }
    }
}

pub enum Direction {
    Left, Right, Up, Down,
}

pub enum Operation {
    Move(Direction),
    Cursor(f64, f64),
    FixCamera,
    None,
}
