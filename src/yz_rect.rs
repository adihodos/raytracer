use super::aabb::Aabb;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct YZRect {
  pub y0: f32,
  pub y1: f32,
  pub z0: f32,
  pub z1: f32,
  pub k: f32,
  pub mtl: Arc<Material>,
}

impl YZRect {
  pub fn new(
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    mtl: Arc<Material>,
  ) -> YZRect {
    YZRect {
      y0,
      y1,
      z0,
      z1,
      k,
      mtl,
    }
  }
}

impl Hitable for YZRect {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let t = (self.k - r.origin.x) / r.direction.x;
    if t < t_min || t > t_max {
      return None;
    }

    let y = r.origin.y + t * r.direction.y;
    let z = r.origin.z + t * r.direction.z;

    if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
      return None;
    }

    Some(HitRecord::new(
      t,
      r.point_at_param(t),
      Vec3::new(1_f32, 0_f32, 0_f32),
      self.mtl.clone(),
      (y - self.y0) / (self.y1 - self.y0),
      (z - self.z0) / (self.z1 - self.z0),
    ))
  }

  fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
    Some(Aabb::new(
      Vec3::new(self.k - 0.0001_f32, self.y0, self.z0),
      Vec3::new(self.k + 0.0001_f32, self.y1, self.z1),
    ))
  }
}
