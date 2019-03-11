use super::hitable::*;
use super::ray::Ray;

pub struct HitableList {
  objects: Vec<Box<Hitable>>,
}

impl HitableList {
  pub fn new() -> HitableList {
    HitableList {
      objects: Vec::new(),
    }
  }

  pub fn add_object(&mut self, obj: Box<Hitable>) {
    self.objects.push(obj);
  }

  pub fn size(&self) -> usize {
    self.objects.len()
  }
}

impl Hitable for HitableList {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut result: Option<HitRecord> = None;

    for obj in self.objects.iter() {
      if let Some(hit_result) = obj.hit(r, t_min, closest_so_far) {
        closest_so_far = hit_result.t;
        result = Some(hit_result);
      }
    }

    result
  }
}
