use super::aabb::Aabb;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct XZRect {
  pub x0: f32,
  pub z0: f32,
  pub x1: f32,
  pub z1: f32,
  pub k: f32,
  pub mtl: Arc<Material>,
}

impl XZRect {
  pub fn new(
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    mtl: Arc<Material>,
  ) -> XZRect {
    XZRect {
      x0,
      x1,
      z0,
      z1,
      k,
      mtl,
    }
  }
}

impl Hitable for XZRect {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let t = (self.k - r.origin.y) / r.direction.y;
    if t < t_min || t > t_max {
      return None;
    }

    let x = r.origin.x + t * r.direction.x;
    let z = r.origin.z + t * r.direction.z;

    if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
      return None;
    }

    Some(HitRecord::new(
      t,
      r.point_at_param(t),
      Vec3::new(0_f32, 1_f32, 0_f32),
      self.mtl.clone(),
      (x - self.x0) / (self.x1 - self.x0),
      (z - self.z0) / (self.z1 - self.z0),
    ))
  }

  fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
    Some(Aabb::new(
      Vec3::new(self.x0, self.k - 0.0001_f32, self.z0),
      Vec3::new(self.x1, self.k + 0.0001_f32, self.z1),
    ))
  }
}
