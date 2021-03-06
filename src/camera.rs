use super::ray::Ray;
use super::vec3::{cross_product, random_in_unit_disk, unit_vector, Vec3};
use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  lens_radius: f32,
  time0: f32,
  time1: f32,
}

#[derive(Copy, Debug, Clone)]
pub struct CameraParameters {
  pub lookfrom: Vec3,
  pub lookat: Vec3,
  pub world_up: Vec3,
  pub field_of_view: f32,
  pub aperture: f32,
  pub focus_dist: f32,
  pub time0: f32,
  pub time1: f32,
}

impl Default for CameraParameters {
  fn default() -> CameraParameters {
    unsafe { std::mem::zeroed() }
  }
}

impl Camera {
  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f32,
    aspect: f32,
    aperture: f32,
    focus_dist: f32,
    time0: f32,
    time1: f32,
  ) -> Camera {
    let lens_radius = aperture * 0.5f32;
    let theta = vfov * std::f32::consts::PI / 180f32;
    let half_height = (theta * 0.5f32).tan();
    let half_width = aspect * half_height;

    let w = unit_vector(lookfrom - lookat);
    let u = unit_vector(cross_product(vup, w));
    let v = cross_product(w, u);

    Camera {
      origin: lookfrom,
      lower_left_corner: lookfrom
        - half_width * focus_dist * u
        - half_height * focus_dist * v
        - focus_dist * w,
      horizontal: 2f32 * half_width * focus_dist * u,
      vertical: 2f32 * half_height * focus_dist * v,
      u,
      v,
      w,
      lens_radius,
      time0,
      time1,
    }
  }

  pub fn ray_at(&self, s: f32, t: f32) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.x + self.v * rd.y;
    let time =
      self.time0 + thread_rng().gen::<f32>() * (self.time1 - self.time0);

    Ray::new(
      self.origin + offset,
      self.lower_left_corner + s * self.horizontal + t * self.vertical
        - self.origin
        - offset,
      time,
    )
  }
}
