//
// todo : try this
// https://gist.github.com/KdotJPG/b1270127455a94ac5d19

const GRADIENTS: [(f32, f32, f32); 12] = [
  (1f32, 1f32, 0f32),
  (-1f32, 1f32, 0f32),
  (1f32, -1f32, 0f32),
  (-1f32, -1f32, 0f32),
  (1f32, 0f32, 1f32),
  (-1f32, 0f32, 1f32),
  (1f32, 0f32, -1f32),
  (-1f32, 0f32, -1f32),
  (0f32, 1f32, 1f32),
  (0f32, -1f32, 1f32),
  (0f32, 1f32, -1f32),
  (0f32, -1f32, -1f32),
];

const PERMUTATIONS: [i32; 256] = [
  151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140,
  36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
  75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237,
  149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48,
  27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105,
  92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73,
  209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
  164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38,
  147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189,
  28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153,
  101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224,
  232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144,
  12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214,
  31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150,
  254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66,
  215, 61, 156, 180,
];

fn fast_floor(x: f32) -> i32 {
  if x > 0f32 {
    x as i32
  } else {
    x as i32 - 1
  }
}

fn dot(g: (f32, f32, f32), x: f32, y: f32) -> f32 {
  g.0 * x + g.1 * y
}

pub struct SimplexNoise {
  perm: Vec<i32>,
  perm_mod12: Vec<i32>,
}

impl SimplexNoise {
  pub fn new() -> SimplexNoise {
    let perm = (0..512)
      .map(|i| PERMUTATIONS[(i & 255) as usize])
      .collect::<Vec<_>>();

    let perm_mod12 = (0..512).map(|i| perm[i] % 12).collect::<Vec<_>>();

    SimplexNoise { perm, perm_mod12 }
  }

  pub fn noise(&self, xin: f32, yin: f32) -> f32 {
    let f2 = 0.5f32 * (3f32.sqrt() - 1f32);
    let s = (xin + yin) * f2;

    let i = fast_floor(xin + s);
    let j = fast_floor(yin + s);

    let g2 = (3f32 - 3f32.sqrt()) / 6f32;

    let t = (i + j) as f32 * g2;

    let X0 = i as f32 - t;
    let Y0 = j as f32 - t;
    let x0 = xin - X0;
    let y0 = yin - Y0;

    let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

    let x1 = x0 - i1 as f32 + g2;
    let y1 = y0 - j1 as f32 + g2;
    let x2 = x0 - 1f32 + 2f32 * g2;
    let y2 = y0 - 1f32 + 2f32 * g2;

    let ii = i & 255;
    let jj = j & 255;

    let gi0 = self.perm_mod12[(ii + self.perm[jj as usize]) as usize];
    let gi1 =
      self.perm_mod12[(ii + i1 + self.perm[(jj + j1) as usize]) as usize];
    let gi2 = self.perm_mod12[(ii + 1 + self.perm[(jj + 1) as usize]) as usize];

    let t0 = 0.5f32 - x0 * x0 - y0 * y0;
    let n0 = if t0 < 0f32 {
      0f32
    } else {
      let t0 = t0 * t0;
      t0 * t0 * dot(GRADIENTS[gi0 as usize], x0, y0)
    };

    let t1 = 0.5f32 - x1 * x1 - y1 * y1;
    let n1 = if t1 < 0f32 {
      0f32
    } else {
      let t1 = t1 * t1;
      t1 * t1 * dot(GRADIENTS[gi1 as usize], x1, y1)
    };

    let t2 = 0.5f32 - x2 * x2 - y2 * y2;
    let n2 = if t2 < 0f32 {
      0f32
    } else {
      let t2 = t2 * t2;
      t2 * t2 * dot(GRADIENTS[gi2 as usize], x2, y2)
    };

    70f32 * (n0 + n1 + n2)
  }
}
