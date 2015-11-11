use std::vec::Vec;
use std::slice::Iter;

use game::operation::*;

const CURSOR_SENSITIVITY: f64 = 1.0;

pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fix_camera: bool,
    cursor_sensitivity: f64,
    cursor: bool,
    pub cursor_pos: [f64; 2],
    pub cursor_diff: [f64; 2],
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            left: false, right: false, up: false, down: false,
            fix_camera: false,
            cursor_sensitivity: CURSOR_SENSITIVITY,
            cursor: false, cursor_pos: [0.0, 0.0], cursor_diff: [0.0, 0.0],
        }
    }

    pub fn cursor(&mut self, x: f64, y: f64) {
        if self.cursor {
            let xy = [x, y];
            for i in 0..2 {
                self.cursor_diff[i] += xy[i] - self.cursor_pos[i];
            }
        }
        self.cursor_pos = [x, y];
        self.cursor = true;
    }

    pub fn press(&mut self, op: Operation) -> &mut Self {
        self.update(op, true)
    }

    pub fn release(&mut self, op: Operation) -> &mut Self {
        self.update(op, false)
    }

    fn update(&mut self, op: Operation, value: bool) -> &mut Self {
        match op {
            Operation::Move(dir) => match dir {
                Direction::Left => self.left = value,
                Direction::Right => self.right = value,
                Direction::Up => self.up = value,
                Direction::Down => self.down = value,
            },
            Operation::FixCamera => self.fix_camera = value,
            _ => {}
        }
        self
    }

    pub fn vec(&mut self) -> Vec<Operation> {
        let mut states = Vec::new();
        if self.left { states.push(Operation::Move(Direction::Left)); }
        if self.right { states.push(Operation::Move(Direction::Right)); }
        if self.up { states.push(Operation::Move(Direction::Up)); }
        if self.down { states.push(Operation::Move(Direction::Down)); }
        states.push(Operation::Cursor(self.cursor_diff[0] * self.cursor_sensitivity, self.cursor_diff[1] * self.cursor_sensitivity));
        self.cursor_diff = [0.0, 0.0];
        states
    }
}
