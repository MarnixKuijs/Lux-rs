pub mod camera;
pub mod geometry;
pub mod material;
pub mod scene;

pub use camera::*;
pub use geometry::*;
pub use material::*;
pub use scene::*;

use cgmath::{ElementWise, InnerSpace, Vector3};
use rand::prelude::*;

use std::f32;
use std::usize;

struct HitRecord {
    hit_distance: f32,
    normal: Vector3<f32>,
    material: Material,
}

struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    fn point_along_ray(&self, t: f32) -> Vector3<f32> {
        return self.origin + t * self.direction;
    }

    fn closest_intersection(&self, scene: &Scene) -> Option<HitRecord> {
        
        let mut closest_hit_distance : f32 = f32::MAX;
        let mut object_position : Vector3<f32> = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut object_index : Option<usize> = None;
        let objects =  &scene.objects();
    
        for (index, object) in objects.iter().enumerate() {
            match object.geometry {
                Geometry::Sphere(position, radius) => {
                    let hypotenuse = position - self.origin;
                    let t = cgmath::dot(hypotenuse, self.direction);
                    let opposite = hypotenuse - t * self.direction;
                    let square_length = cgmath::dot(opposite, opposite);
                    let square_radius = radius * radius;
                    if square_length > square_radius || t < 0.001 {
                        continue;
                    }
                    let hit_distance = t - (square_radius - square_length).sqrt();
                    if hit_distance < closest_hit_distance {
                        closest_hit_distance = hit_distance;
                        object_index = Some(index);
                        object_position = position; 
                    }
                }
            }
        }

        match object_index {
            Some(index) => {
                return Some(HitRecord {
                hit_distance: closest_hit_distance,
                normal: (self.point_along_ray(closest_hit_distance) - object_position).normalize(),
                material: objects[index].material
                }); 
            },
            None => {return None;}
        }
    }

    fn sample_skybox(&self) -> Vector3<f32> {
        let color = (1.0 - (0.5 * (self.direction.y + 1.0))) * Vector3::new(1.0, 1.0, 1.0)
            + (0.5 * (self.direction.y + 1.0)) * Vector3::new(0.5, 0.7, 1.0);
        return Vector3::new(color.x, color.y, color.z);
    }
}

fn random_in_unit_sphere() -> Vector3<f32> {
    let mut p: Vector3<f32>;
    let mut rng = rand::thread_rng();
    loop {
        p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vector3::new(1.0, 1.0, 1.0);
        if cgmath::dot(p, p) >= 1.0 {
            break;
        }
    }
    return p;
}

const MAX_RECURSION_COUNT: u32 = 50;

fn trace(scene: &Scene, ray: &Ray, recusion_count: u32) -> Vector3<f32> {
    let option = ray.closest_intersection(scene);
    match option {
        None => {
            return ray.sample_skybox();
        }
        Some(hit_record) => {
            let option = scatter(ray, &hit_record);
            if let (Option::Some((scattered_ray, attenuation)), Some(true)) =
                (option, Some(recusion_count < MAX_RECURSION_COUNT))
            {
                return trace(scene, &scattered_ray, recusion_count + 1)
                    .mul_element_wise(attenuation);
            } else {
                return Vector3::new(0.0, 0.0, 0.0);
            }
        }
    }
}

fn scatter(hit_ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
    match hit_record.material {
        Material::Lambert(albedo) => {
            let hit_point = hit_ray.point_along_ray(hit_record.hit_distance);
            let target = hit_record.normal + random_in_unit_sphere();
            let scattered_ray = Ray {
                origin: hit_point,
                direction: target.normalize(),
            };
            let attenuation = albedo;

            return Some((scattered_ray, attenuation));
        }
        Material::Metalic(albedo, fuzz) => {
            let hit_point = hit_ray.point_along_ray(hit_record.hit_distance);
            let reflect_direction = reflect(&hit_ray.direction, &hit_record.normal);
            let scattered_ray = Ray {
                origin: hit_point,
                direction: reflect_direction + fuzz.min(1.0) * random_in_unit_sphere(),
            };
            if cgmath::dot(scattered_ray.direction, hit_record.normal) > 0.0 {
                return Some((scattered_ray, albedo));
            }
            return None;
        }
    }
}

fn reflect(direction: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    return direction - 2.0 * cgmath::dot(*direction, *normal) * normal;
}

pub fn render(
    scene: &Scene,
    camera: &Camera,
    image_width: u32,
    image_height: u32,
    num_samples: u32,
) {
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    let mut rng = rand::thread_rng();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut color: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        for _sample in 0..num_samples {
            let u = (x as f32 + rng.gen::<f32>()) / image_width as f32;
            let v = ((image_height - y) as f32 + rng.gen::<f32>()) / image_height as f32;

            let mut direction = camera.lower_left_corner;
            direction.x += u * camera.width;
            direction.y += v * camera.height;

            let ray = Ray {
                origin: camera.position,
                direction: direction.normalize(),
            };

            color += trace(scene, &ray, 0);
        }
        color /= num_samples as f32;
        color = Vector3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
        color *= 255.99;

        *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
    }

    imgbuf.save("bin/image.png").unwrap();
}
