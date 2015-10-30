use graphics::math::{ Scalar, Vec2d };

pub struct Camera {
    position: Vec2d,
    rotation: Scalar,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0.0, 0.0],
            rotation: 0.0,
        }
    }
}
