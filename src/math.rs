use graphics::math::{ Scalar, Vec2d, Matrix2d, identity };

pub fn rotate_vec(vec: Vec2d, rad: Scalar) -> Vec2d {
    let cos = rad.cos();
    let sin = rad.sin();
    [vec[0] * cos + vec[1] * sin, - vec[0] * sin + vec[1] * cos]
}
