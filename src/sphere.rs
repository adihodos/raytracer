use super::aabb::Aabb;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::{dot_product, Vec3};
use std::sync::Arc;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub mtl: Arc<Material>,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, mtl: Arc<Material>) -> Sphere {
    Sphere {
      center,
      radius,
      mtl,
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = dot_product(r.direction, r.direction);
    let b = dot_product(oc, r.direction);
    let c = dot_product(oc, oc) - self.radius * self.radius;
    let delta = b * b - a * c;

    if delta > 0f32 {
      let temp = (-b - (b * b - a * c).sqrt()) / a;

      if (temp < t_max) && (temp > t_min) {
        let p = r.point_at_param(temp);
        let n = (p - self.center) / self.radius;

        return Some(HitRecord::new(temp, p, n, self.mtl.clone()));
      }

      let temp = (-b + (b * b - a * c).sqrt()) / a;
      if (temp < t_max) && (temp > t_min) {
        let p = r.point_at_param(temp);
        let n = (p - self.center) / self.radius;

        return Some(HitRecord::new(temp, p, n, self.mtl.clone()));
      }
    }

    None
  }

  fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
    Some(Aabb::new(
      self.center - Vec3::same(self.radius),
      self.center + Vec3::same(self.radius),
    ))
  }
}
