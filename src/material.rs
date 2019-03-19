use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub trait Material: Send + Sync {
  fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<(Vec3, Ray)>;

  fn emitted(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
    Vec3::same(0_f32)
  }
}
