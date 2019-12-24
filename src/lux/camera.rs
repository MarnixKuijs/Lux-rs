use cgmath::Vector3;
use std::f32::consts::PI;

pub struct Camera {
    pub position: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub width: Vector3<f32>,
    pub height: Vector3<f32>,
}

impl Camera {
    pub fn new(
        position: Vector3<f32>,
        lookat: Vector3<f32>,
        up: Vector3<f32>,
        fov: f32,
        aspect: f32,
    ) -> Camera {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = cgmath::InnerSpace::normalize(position - lookat);
        let u = cgmath::InnerSpace::normalize(cgmath::Vector3::cross(up, w));
        let v = cgmath::Vector3::cross(w, u);

        return Camera {
            position: position,
            lower_left_corner: position - half_width * u - half_height * v - w,
            width: 2.0 * half_width * u,
            height: 2.0 * half_height * v,
        };
    }
}
