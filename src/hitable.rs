use super::aabb::Aabb;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub mtl: Arc<Material>,
  pub u: f32,
  pub v: f32,
}

impl HitRecord {
  pub fn new(
    t: f32,
    p: Vec3,
    normal: Vec3,
    mtl: Arc<Material>,
    u: f32,
    v: f32,
  ) -> HitRecord {
    HitRecord {
      t,
      p,
      normal,
      mtl,
      u,
      v,
    }
  }
}

pub trait Hitable: Send + Sync {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
  fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}
