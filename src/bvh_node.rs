use super::aabb::Aabb;
use super::hitable::HitRecord;
use super::hitable::Hitable;
use super::ray::Ray;
use rand::prelude::*;
use std::sync::Arc;

pub struct BvhNode {
  pub bbox: Aabb,
  pub left: Arc<Hitable>,
  pub right: Arc<Hitable>,
}

impl Hitable for BvhNode {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    if self.bbox.hit(r, t_min, t_max) {
      let hit_left_rec = self.left.hit(r, t_min, t_max);
      let hit_right_rec = self.right.hit(r, t_min, t_max);

      if hit_left_rec.is_some() && hit_right_rec.is_some() {
        let lrec = hit_left_rec.unwrap();
        let rrec = hit_right_rec.unwrap();

        if lrec.t < rrec.t {
          return Some(lrec);
        } else {
          return Some(rrec);
        }
      } else if hit_left_rec.is_some() {
        return hit_left_rec;
      } else if hit_right_rec.is_some() {
        return hit_right_rec;
      } else {
        return None;
      }
    }

    None
  }

  fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
    Some(self.bbox)
  }
}

impl BvhNode {
  pub fn new(l: &mut [Arc<Hitable>], time0: f32, time1: f32) -> Arc<BvhNode> {
    let mut rng = thread_rng();

    let axis = (3_f32 * rng.gen::<f32>()) as i32;

    match axis {
      0 => l.sort_by(|a, b| {
        if let Some(a_box) = a.bounding_box(0_f32, 0_f32) {
          if let Some(b_box) = b.bounding_box(0_f32, 0_f32) {
            a_box.min.x.partial_cmp(&b_box.min.x).unwrap()
          } else {
            std::cmp::Ordering::Equal
          }
        } else {
          std::cmp::Ordering::Equal
        }
      }),
      1 => l.sort_by(|a, b| {
        if let Some(a_box) = a.bounding_box(0_f32, 0_f32) {
          if let Some(b_box) = b.bounding_box(0_f32, 0_f32) {
            a_box.min.y.partial_cmp(&b_box.min.y).unwrap()
          } else {
            std::cmp::Ordering::Equal
          }
        } else {
          std::cmp::Ordering::Equal
        }
      }),
      _ => l.sort_by(|a, b| {
        if let Some(a_box) = a.bounding_box(0_f32, 0_f32) {
          if let Some(b_box) = b.bounding_box(0_f32, 0_f32) {
            a_box.min.z.partial_cmp(&b_box.min.z).unwrap()
          } else {
            std::cmp::Ordering::Equal
          }
        } else {
          std::cmp::Ordering::Equal
        }
      }),
    }

    let n = l.len();

    let (left, right) = match l.len() {
      1 => (l[0].clone(), l[0].clone()),
      2 => (l[0].clone(), l[1].clone()),
      _ => (
        BvhNode::new(&mut l[0..n / 2], time0, time1) as Arc<Hitable>,
        BvhNode::new(&mut l[n / 2..], time0, time1) as Arc<Hitable>,
      ),
    };

    let box_left = left.bounding_box(time0, time1);
    let box_right = right.bounding_box(time0, time1);

    let bbox = if box_left.is_some() && box_right.is_some() {
      Aabb::merge(&box_left.unwrap(), &box_right.unwrap())
    } else if box_left.is_some() {
      box_left.unwrap()
    } else {
      box_right.unwrap()
    };

    Arc::new(BvhNode { bbox, left, right })
  }
}
