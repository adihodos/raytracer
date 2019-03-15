use super::vec3::Vec3;
use super::texture::Texture;
use std::sync::Arc;

pub struct CheckerTexture {
  odd : Arc<Texture>,
  even : Arc<Texture>
}

impl CheckerTexture {
  pub fn new(odd : Arc<Texture>, even : Arc<Texture>) -> CheckerTexture {
    CheckerTexture{odd, even}
  }
}

impl Texture for CheckerTexture {
  fn value(&self, u : f32, v : f32, p : Vec3) -> Vec3 {
    let sines = (10_f32 * p.x).sin() * (10_f32 * p.y).sin() * (10_f32 * p.z).sin();
    if sines < 0_f32 {
      self.odd.value(u, v, p)
    } else {
      self.even.value(u, v, p)
    }
  }
}