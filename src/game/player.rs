use graphics::math::{ Scalar };

pub struct Player {
    position: (Scalar, Scalar),
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: (0., 0.),
        }
    }
}
