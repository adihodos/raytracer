use super::ray::Ray;
use super::vec3::Vec3;

// fn ffmin(a: f32, b: f32) -> f32 {
//   if a < b {
//     a
//   } else {
//     b
//   }
// }

// fn ffmax(a: f32, b: f32) -> f32 {
//   if a > b {
//     a
//   } else {
//     b
//   }
// }

pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3,
}

impl Aabb {
  pub fn new(min: Vec3, max: Vec3) -> Aabb {
    Aabb { min, max }
  }

  pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
    let mut tmin = tmin;
    let mut tmax = tmax;

    for a in 0..3 {
      let invd = 1_f32 / r.direction[a as usize];
      let mut t0 = invd * (self.min[a as usize] - r.origin[a as usize]);
      let mut t1 = invd * (self.max[a as usize] - r.origin[a as usize]);

      if invd < 0_f32 {
        t0 = std::mem::replace(&mut t1, t0);
      }

      tmin = if t0 > tmin { t0 } else { tmin };
      tmax = if t1 < tmax { t1 } else { tmax };

      if tmax <= tmin {
        return false;
      }
    }

    true
  }

  pub fn merge(a: &Aabb, b: &Aabb) -> Aabb {
    let min = Vec3::new(
      a.min.x.min(b.min.x),
      a.min.y.min(b.min.y),
      a.min.z.min(b.min.z),
    );

    let max = Vec3::new(
      a.max.x.max(b.max.x),
      a.max.y.max(b.max.y),
      a.max.z.max(b.max.z),
    );

    Aabb::new(min, max)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_aabb_merge() {
    let box0 = Aabb::new(Vec3::same(0_f32), Vec3::same(3_f32));
    let box1 = Aabb::new(Vec3::same(-3_f32), Vec3::same(1_f32));

    let big_box = Aabb::merge(&box0, &box1);
    assert_eq!(big_box.min, Vec3::same(-3_f32));
    assert_eq!(big_box.max, Vec3::same(3_f32));
  }
}
