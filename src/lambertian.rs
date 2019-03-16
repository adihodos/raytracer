use super::hitable::HitRecord;
use super::material::Material;
use super::ray::Ray;
use super::texture::Texture;
use super::vec3::{random_in_unit_sphere, Vec3};
use std::sync::Arc;

pub struct Lambertian {
  pub albedo: Arc<Texture>,
}

impl Lambertian {
  pub fn new(albedo: Arc<Texture>) -> Lambertian {
    Lambertian { albedo }
  }
}

impl Material for Lambertian {
  fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<(Vec3, Ray)> {
    let target = h.p + h.normal + random_in_unit_sphere();
    let scattered = Ray::new(h.p, target - h.p, r.time);
    let attenuation = self.albedo.value(0_f32, 0_f32, h.p);

    Some((attenuation, scattered))
  }
}
