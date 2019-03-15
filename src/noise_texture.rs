use super::perlin::PerlinNoise;
use super::texture::Texture;
use super::vec3::Vec3;

pub struct NoiseTexture {
  noise: PerlinNoise,
  scale: f32,
}

impl NoiseTexture {
  pub fn new(scale: f32) -> NoiseTexture {
    NoiseTexture {
      noise: PerlinNoise::new(),
      scale,
    }
  }
}

impl Texture for NoiseTexture {
  fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
    Vec3::same(1_f32) * self.noise.noise(self.scale * p)
  }
}
