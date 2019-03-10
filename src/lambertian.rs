use super::hitable::HitRecord;
use super::material::Material;
use super::ray::Ray;
use super::vec3::{random_in_unit_sphere, Vec3};

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, h: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = h.p + h.normal + random_in_unit_sphere();
        let scattered = Ray::new(h.p, target - h.p);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}
