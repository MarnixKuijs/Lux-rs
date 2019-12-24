use cgmath::Vector3;

pub struct Camera {
    pub position: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub width: f32,
    pub height: f32,
}
