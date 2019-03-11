use super::hitable::HitRecord;
use super::material::Material;
use super::ray::Ray;
use super::vec3::{dot_product, reflect, refract, schlick, Vec3};
use rand::prelude::*;

pub struct Dielectric {
  pub ref_idx: f32,
}

impl Dielectric {
  pub fn new(ri: f32) -> Dielectric {
    Dielectric { ref_idx: ri }
  }
}

impl Material for Dielectric {
  fn scatter(&self, r: &Ray, h: &HitRecord) -> Option<(Vec3, Ray)> {
    let reflected = reflect(r.direction, h.normal);
    let attenuation = Vec3::same(1_f32);

    let (outward_normal, ni_over_nt, cosine) =
      if dot_product(r.direction, h.normal) > 0f32 {
        (
          -h.normal,
          self.ref_idx,
          self.ref_idx * dot_product(r.direction, h.normal)
            / r.direction.length(),
        )
      } else {
        (
          h.normal,
          1f32 / self.ref_idx,
          -dot_product(r.direction, h.normal) / r.direction.length(),
        )
      };

    let (refracted, reflect_prob) =
      if let Some(refr) = refract(r.direction, outward_normal, ni_over_nt) {
        (Some(refr), schlick(cosine, self.ref_idx))
      } else {
        (None, 1f32)
      };

    if thread_rng().gen::<f32>() < reflect_prob {
      Some((attenuation, Ray::new(h.p, reflected)))
    } else {
      Some((attenuation, Ray::new(h.p, refracted.unwrap())))
    }
  }
}
