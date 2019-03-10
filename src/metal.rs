use super::hitable::HitRecord;
use super::material::Material;
use super::ray::Ray;
use super::vec3::{dot_product, random_in_unit_sphere, reflect, unit_vector, Vec3};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Metal {
        let fuzz = if f < 1f32 { f } else { 1f32 };

        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(unit_vector(r.direction), h.normal);
        let scattered = Ray::new(h.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;

        if dot_product(scattered.direction, h.normal) > 0f32 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
