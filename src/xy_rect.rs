use super::aabb::Aabb;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct XYRect {
  pub x0: f32,
  pub x1: f32,
  pub y0: f32,
  pub y1: f32,
  pub k: f32,
  pub mtl: Arc<Material>,
}

impl XYRect {
  pub fn new(
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    mtl: Arc<Material>,
  ) -> XYRect {
    XYRect {
      x0,
      x1,
      y0,
      y1,
      k,
      mtl,
    }
  }
}

impl Hitable for XYRect {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let t = (self.k - r.origin.z) / r.direction.z;
    if t < t_min || t > t_max {
      return None;
    }

    let x = r.origin.x + t * r.direction.x;
    let y = r.origin.y + t * r.direction.y;

    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
      return None;
    }

    Some(HitRecord::new(
      t,
      r.point_at_param(t),
      Vec3::new(0_f32, 0_f32, 1_f32),
      self.mtl.clone(),
      (x - self.x0) / (self.x1 - self.x0),
      (y - self.y0) / (self.y1 - self.y0),
    ))
  }

  fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
    Some(Aabb::new(
      Vec3::new(self.x0, self.y0, self.k - 0.0001_f32),
      Vec3::new(self.x1, self.y1, self.k + 0.0001_f32),
    ))
  }
}
