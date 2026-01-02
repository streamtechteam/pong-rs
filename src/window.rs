use raylib::math::Vector2;

#[derive(Default)]
pub struct Window {
    pub width: i32,
    pub width_slice: i32,
    pub height: i32,
    pub height_slice: i32,
    pub center: Vector2,
}
