mod lux;

use cgmath::Vector3;
use std::fs;

const SCR_WIDTH: u32 = 1000;
const SCR_HEIGHT: u32 = 500;
const NUMBER_SAMPLES: u32 = 100;

fn main() {
    fs::create_dir_all("bin/").expect("Coudn't create directory");

    let mut scene = lux::Scene::new();
    let camera = lux::Camera {
        position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        lower_left_corner: Vector3 {
            x: -2.0,
            y: -1.0,
            z: -1.0,
        },
        width: 4.0,
        height: 2.0,
    };

    let object1 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(0.0, 0.0, -1.0), 0.5),
        material: lux::Material::Lambert(Vector3::new(0.8, 0.3, 0.3)),
    };
    let object2 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(0.0, -100.5, -1.0), 100.0),
        material: lux::Material::Lambert(Vector3::new(0.8, 0.8, 0.0)),
    };
    let object3 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(1.0, 0.0, -1.0), 0.5),
        material: lux::Material::Metalic(Vector3::new(0.8, 0.6, 0.2), 1.0),
    };
    let object4 = lux::Object {
        geometry: lux::Geometry::Sphere(Vector3::new(-1.0, 0.0, -1.0), 0.5),
        material: lux::Material::Metalic(Vector3::new(0.8, 0.8, 0.8), 0.3),
    };

    scene.add_object(object1);
    scene.add_object(object2);
    scene.add_object(object3);
    scene.add_object(object4);

    lux::render(&scene, &camera, SCR_WIDTH, SCR_HEIGHT, NUMBER_SAMPLES);
}
