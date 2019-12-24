mod lux;

use cgmath::Vector3;
use std::fs;

const SCR_WIDTH: u32 = 2000;
const SCR_HEIGHT: u32 = 1000;
const NUMBER_SAMPLES: u32 = 1000;

fn main() {
    fs::create_dir_all("bin/").expect("Coudn't create directory");
    let mut scene = lux::Scene::new();
    let camera = lux::Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        SCR_WIDTH as f32 / SCR_HEIGHT as f32,
    );

    let object1 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(0.0, 0.0, -1.0), 0.5),
        material: lux::Material::Lambert(Vector3::new(1.0, 0.0, 1.0)),
    };
    let object2 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(0.0, -100.5, -1.0), 100.0),
        material: lux::Material::Lambert(Vector3::new(0.0, 0.8, 0.0)),
    };
    let object3 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(1.0, 0.0, -1.0), 0.5),
        material: lux::Material::Metalic(Vector3::new(0.4, 0.4, 0.4), 0.0),
    };
    let object4 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(-1.0, 0.0, -1.0), 0.5),
        material: lux::Material::Dielectric(1.5),
    };
    let object5 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(-1.0, 0.0, -1.0), -0.45),
        material: lux::Material::Dielectric(1.5),
    };

    scene.add_object(object1);
    scene.add_object(object2);
    scene.add_object(object3);
    scene.add_object(object4);
    scene.add_object(object5);

    lux::render(&scene, &camera, SCR_WIDTH, SCR_HEIGHT, NUMBER_SAMPLES);
}
