use super::hitable::HitRecord;
use super::material::Material;
use super::ray::Ray;
use super::texture::Texture;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct DiffuseLight {
  emit: Arc<Texture>,
}

impl DiffuseLight {
  pub fn new(tex: Arc<Texture>) -> DiffuseLight {
    DiffuseLight { emit: tex }
  }
}

impl Material for DiffuseLight {
  fn scatter(&self, _r: &Ray, _h: &HitRecord) -> Option<(Vec3, Ray)> {
    None
  }

  fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
    self.emit.value(u, v, p)
  }
}
