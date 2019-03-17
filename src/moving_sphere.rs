use super::aabb::Aabb;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::{dot_product, Vec3};
use std::sync::Arc;

pub struct MovingSphere {
  pub center0: Vec3,
  pub center1: Vec3,
  pub radius: f32,
  pub mtl: Arc<Material>,
  pub time0: f32,
  pub time1: f32,
}

impl MovingSphere {
  pub fn new(
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    mtl: Arc<Material>,
  ) -> MovingSphere {
    MovingSphere {
      center0,
      center1,
      radius,
      mtl,
      time0,
      time1,
    }
  }

  pub fn center(&self, time: f32) -> Vec3 {
    self.center0
      + ((time - self.time0) / (self.time1 - self.time0))
        * (self.center1 - self.center0)
  }
}

impl Hitable for MovingSphere {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = r.origin - self.center(r.time);
    let a = dot_product(r.direction, r.direction);
    let b = dot_product(oc, r.direction);
    let c = dot_product(oc, oc) - self.radius * self.radius;
    let delta = b * b - a * c;

    if delta > 0f32 {
      let temp = (-b - (b * b - a * c).sqrt()) / a;

      if (temp < t_max) && (temp > t_min) {
        let p = r.point_at_param(temp);
        let n = (p - self.center(r.time)) / self.radius;

        return Some(HitRecord::new(temp, p, n, self.mtl.clone()));
      }

      let temp = (-b + (b * b - a * c).sqrt()) / a;
      if (temp < t_max) && (temp > t_min) {
        let p = r.point_at_param(temp);
        let n = (p - self.center(r.time)) / self.radius;

        return Some(HitRecord::new(temp, p, n, self.mtl.clone()));
      }
    }

    None
  }

  fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
    //        Some(Aabb::new(self.center))
    None
  }
}
