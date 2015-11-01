use graphics::math::{ Scalar, Vec2d, Matrix2d, identity };
use graphics::Transformed;

pub struct Camera {
    pub position: Vec2d,
    pub rotation: Scalar,
    pub scale: Scalar,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0.0, 0.0],
            rotation: 0.0,
            scale: 10.0
        }
    }

    pub fn get_transform(&self) -> Matrix2d {
        identity().rot_rad(self.rotation).scale(self.scale, self.scale).trans(- self.position[0], - self.position[1])
    }

    pub fn move_by(&mut self, m: Vec2d) -> &mut Self {
        for i in 0..2 {
            self.position[i] = self.position[i] + m[i]
        }
        self
    }

    pub fn move_to(&mut self, m: Vec2d) -> &mut Self {
        for i in 0..2 {
            self.position[i] = m[i]
        }
        self
    }

    pub fn rotate(&mut self, r: Scalar) -> &mut Self {
        self.rotation = self.rotation + r;
        self
    }

    pub fn set_rotation(&mut self, r: Scalar) -> &mut Self {
        self.rotation = r;
        self
    }

    pub fn scale(&mut self, s: Scalar) -> &mut Self {
        self.scale = self.scale + s;
        self
    }

    pub fn set_scale(&mut self, s: Scalar) -> &mut Self {
        self.scale = s;
        self
    }
}
