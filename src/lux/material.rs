use cgmath::Vector3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambert(Vector3<f32>),
    Metalic(Vector3<f32>, f32),
}
