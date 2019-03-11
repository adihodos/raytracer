use rand::prelude::*;
use rgb::RGB8;
use std::ops::{
  Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn same(v: f32) -> Vec3 {
    Vec3::new(v, v, v)
  }

  pub fn squared_length(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn length(&self) -> f32 {
    self.squared_length().sqrt()
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Vec3) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Vec3 {
    let mut res = self;
    res += rhs;
    res
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Vec3) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Vec3 {
    let mut res = self;
    res -= rhs;
    res
  }
}

impl Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Vec3 {
    Vec3::new(-self.x, -self.y, -self.z)
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, k: f32) {
    self.x /= k;
    self.y /= k;
    self.z /= k;
  }
}

impl Div<f32> for Vec3 {
  type Output = Vec3;

  fn div(self, k: f32) -> Vec3 {
    let mut res = self;
    res /= k;
    res
  }
}

impl DivAssign for Vec3 {
  fn div_assign(&mut self, rhs: Vec3) {
    self.x /= rhs.x;
    self.y /= rhs.y;
    self.z /= rhs.z;
  }
}

impl Div for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: Vec3) -> Vec3 {
    let mut res = self;
    res /= rhs;
    res
  }
}

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, k: f32) {
    self.x *= k;
    self.y *= k;
    self.z *= k;
  }
}

impl Mul<f32> for Vec3 {
  type Output = Vec3;

  fn mul(self, k: f32) -> Vec3 {
    let mut res = self;
    res *= k;
    res
  }
}

impl MulAssign for Vec3 {
  fn mul_assign(&mut self, rhs: Vec3) {
    self.x *= rhs.x;
    self.y *= rhs.y;
    self.z *= rhs.z;
  }
}

impl Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    let mut res = self;
    res *= rhs;
    res
  }
}

impl Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, v: Vec3) -> Vec3 {
    v * self
  }
}

pub fn dot_product(a: Vec3, b: Vec3) -> f32 {
  a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross_product(a: Vec3, b: Vec3) -> Vec3 {
  Vec3::new(
    a.y * b.z - a.z * b.y,
    a.z * b.x - b.z * a.x,
    a.x * b.y - b.x * a.y,
  )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  v / v.length()
}

pub fn to_rgb8(v: Vec3) -> RGB8 {
  RGB8::new(
    (v.x * 255.99f32) as u8,
    (v.y * 255.99f32) as u8,
    (v.z * 255.99f32) as u8,
  )
}

pub fn random_in_unit_sphere() -> Vec3 {
  let mut rng = thread_rng();

  loop {
    let x: f32 = rng.gen();
    let y: f32 = rng.gen();
    let z: f32 = rng.gen();

    let v = Vec3::new(x, y, z);

    if v.squared_length() < 1f32 {
      return v;
    }
  }
}

pub fn random_in_unit_disk() -> Vec3 {
  let mut rng = thread_rng();

  loop {
    let p = 2f32 * Vec3::new(rng.gen(), rng.gen(), 0f32)
      - Vec3::new(1f32, 1f32, 0f32);
    if dot_product(p, p) < 1f32 {
      return p;
    }
  }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - 2f32 * dot_product(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
  let uv = unit_vector(v);
  let dt = dot_product(uv, n);
  let discriminant = 1f32 - ni_over_nt * ni_over_nt * (1f32 - dt * dt);

  if discriminant > 0f32 {
    Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
  } else {
    None
  }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let r0 = (1f32 - ref_idx) / (1f32 + ref_idx);
  let r0 = r0 * r0;
  r0 + (1f32 - r0) * (1f32 - cosine).powf(5f32)
}
