use super::vec3::Vec3;
use super::texture::Texture;

pub struct ConstantTexture {
  color : Vec3
}

impl ConstantTexture {
  pub fn new(c : Vec3) -> ConstantTexture {
    ConstantTexture{color : c}
  }
}

impl Texture for ConstantTexture {
  fn value(&self, _u : f32, _v : f32, _p : Vec3) -> Vec3 {
    self.color
  }
}