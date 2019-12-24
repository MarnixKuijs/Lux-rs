use super::Geometry;
use super::Material;

pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
}

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            objects: Vec::new(),
        };
    }

    pub fn add_object(&mut self, object: Object) -> () {
        self.objects.push(object);
    }

    pub fn objects(&self) -> &Vec<Object> {
        return &self.objects;
    }
}
