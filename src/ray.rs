use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
  pub time: f32,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
    Ray {
      origin,
      direction,
      time,
    }
  }

  pub fn point_at_param(self, t: f32) -> Vec3 {
    self.origin + self.direction * t
  }
}
