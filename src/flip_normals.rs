use super::aabb::Aabb;
use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;
use std::sync::Arc;

pub struct FlipNormals {
  obj: Arc<Hitable>,
}

impl FlipNormals {
  pub fn new(obj: Arc<Hitable>) -> FlipNormals {
    FlipNormals { obj }
  }
}

impl Hitable for FlipNormals {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    if let Some(hit) = self.obj.hit(r, t_min, t_max) {
      Some(HitRecord::new(
        hit.t,
        hit.p,
        hit.normal * -1f32,
        hit.mtl.clone(),
        hit.u,
        hit.v,
      ))
    } else {
      None
    }
  }

  fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
    self.obj.bounding_box(t0, t1)
  }
}
