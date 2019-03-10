use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mtl: Rc<Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, mtl: Rc<Material>) -> HitRecord {
        HitRecord { t, p, normal, mtl }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
