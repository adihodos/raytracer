extern crate png;
extern crate rand;
extern crate rgb;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use rgb::RGB8;

mod aabb;
mod bvh_node;
mod camera;
mod checker_texture;
mod constant_texture;
mod dielectric;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod moving_sphere;
mod noise_texture;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod timer;
mod vec3;
mod window;

use camera::Camera;
use checker_texture::CheckerTexture;
use constant_texture::ConstantTexture;
use dielectric::Dielectric;
use hitable::*;
use hitable_list::HitableList;
use lambertian::Lambertian;
use material::Material;
use metal::Metal;
use moving_sphere::MovingSphere;
use noise_texture::NoiseTexture;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use timer::BasicTimer;
use vec3::{to_rgb8, Vec3};
use window::Window;

fn write_image(
  filename: &str,
  width: u32,
  height: u32,
  pixels: &[RGB8],
) -> std::io::Result<()> {
  use png::HasParameters;
  use std::fs::File;
  use std::io::BufWriter;

  let file = File::create(filename)?;
  let writer = BufWriter::new(file);

  let mut encoder = png::Encoder::new(writer, width, height);
  encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
  let mut png_writer = encoder.write_header()?;

  let img_data = unsafe {
    std::slice::from_raw_parts(pixels.as_ptr() as *const u8, pixels.len() * 3)
  };

  png_writer.write_image_data(&img_data)?;

  Ok(())
}

fn color(r: &Ray, world: &HitableList, depth: i32) -> Vec3 {
  if let Some(hit) = world.hit(r, 0.001f32, std::f32::MAX) {
    if depth < 50 {
      if let Some((attn, scattered)) = hit.mtl.scatter(r, &hit) {
        return attn * color(&scattered, world, depth + 1);
      }
    }

    return Vec3::same(0f32);
  }

  let unit_direction = vec3::unit_vector(r.direction);
  let t = 0.5f32 * (unit_direction.y + 1f32);
  (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1f32)
}

fn random_scene() -> (HitableList, Vec<Arc<Material>>) {
  let mut world = HitableList::new();
  let mut materials: Vec<Arc<Material>> = Vec::new();

  let checker_tex = Arc::new(CheckerTexture::new(
    Arc::new(ConstantTexture::new(Vec3::new(0.2_f32, 0.3_f32, 0.1_f32))),
    Arc::new(ConstantTexture::new(Vec3::new(0.9_f32, 0.9_f32, 0.9_f32))),
  ));

  materials.push(Arc::new(Lambertian::new(checker_tex)));

  let noise_tex = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(8_f32))));

  world.add_object(Box::new(Sphere::new(
    Vec3::new(0f32, -1000f32, 0f32),
    1000f32,
    // materials[0].clone(),
    noise_tex.clone(),
  )));

  let mut rng = thread_rng();

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = rng.gen::<f32>();

      let center = Vec3::new(
        a as f32 + 0.9f32 * rng.gen::<f32>(),
        0.2f32,
        b as f32 + 0.9f32 * rng.gen::<f32>(),
      );

      if (center - Vec3::new(4f32, 0.2f32, 0f32)).length() > 0.9f32 {
        let mtl: Arc<Material> = if choose_mat < 0.8f32 {
          //
          // diffuse
          let texture = Arc::new(ConstantTexture::new(Vec3::new(
            rng.gen::<f32>() * rng.gen::<f32>(),
            rng.gen::<f32>() * rng.gen::<f32>(),
            rng.gen::<f32>() * rng.gen::<f32>(),
          )));

          Arc::new(Lambertian::new(texture))
        } else if choose_mat < 0.95f32 {
          //
          // metal
          Arc::new(Metal::new(
            Vec3::new(
              0.5f32 * (1f32 + rng.gen::<f32>()),
              0.5f32 * (1f32 + rng.gen::<f32>()),
              0.5f32 * (1f32 + rng.gen::<f32>()),
            ),
            0.5f32 * rng.gen::<f32>(),
          ))
        } else {
          //
          // glass
          Arc::new(Dielectric::new(1.5f32))
        };

        materials.push(mtl.clone());
        world.add_object(Box::new(Sphere::new(center, 0.2f32, mtl.clone())));
      }
    }
  }

  let mtl = Arc::new(Dielectric::new(1.5f32));
  materials.push(mtl.clone());
  world.add_object(Box::new(Sphere::new(
    Vec3::new(0f32, 1f32, 0f32),
    1f32,
    mtl.clone(),
  )));

  let mtl = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
    Vec3::new(0.4f32, 0.2f32, 0.1f32),
  ))));
  materials.push(mtl.clone());
  world.add_object(Box::new(Sphere::new(
    Vec3::new(-4f32, 1f32, 0f32),
    1f32,
    mtl.clone(),
  )));

  let mtl = Arc::new(Metal::new(Vec3::new(0.7f32, 0.6f32, 0.5f32), 0f32));
  materials.push(mtl.clone());
  world.add_object(Box::new(Sphere::new(
    Vec3::new(4f32, 1f32, 0f32),
    1f32,
    mtl.clone(),
  )));

  (world, materials)
}

const THREAD_COUNT: i32 = 4;
const WORK_TILE_SIZE: u32 = 4;

fn test_perlin() {
  // const IMG_WIDTH: u32 = 1024;
  // const IMG_HEIGHT: u32 = 1024;
  // const FEATURE_SIZE: u32 = 128;

  // let mut pixels: Vec<RGB8> = Vec::new();

  // let noise_gen = perlin::SimplexNoise::new();

  // for y in 0..IMG_HEIGHT {
  //   for x in 0..IMG_WIDTH {
  //     let s = x as f32 / FEATURE_SIZE as f32;
  //     let t = y as f32 / FEATURE_SIZE as f32;

  //     let pixel = ((noise_gen.noise(s, t) + 1f32) * 127.5f32) as u8;
  //     pixels.push(RGB8::new(pixel, 0, pixel));
  //   }
  // }

  // write_image("simplex.noise.png", IMG_WIDTH, IMG_HEIGHT, &pixels)
  //   .expect("Failed to write image!");
}

fn main() {
  // test_perlin();

  let nx = 1200;
  let ny = 800;
  //let ns = 64;
  const RAYS_PER_PIXEL: u32 = 16;

  let window = Window::new(0, nx, 0, ny);
  let lookfrom = Vec3::new(13f32, 2f32, 3f32);
  let lookat = Vec3::new(0f32, 0f32, 0f32);
  let dist_to_focus = 10f32;
  let aperture = 0.1f32;

  let cam = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0f32, 1f32, 0f32),
    20f32,
    nx as f32 / ny as f32,
    aperture,
    dist_to_focus,
    0_f32,
    1_f32,
  );

  let (world, _) = random_scene();
  let world = Arc::new(world);

  let domains = {
    let mut d = Vec::new();
    let work_x = window.width() / WORK_TILE_SIZE;
    let work_y = window.height() / WORK_TILE_SIZE;

    println!("Work_x {} :: Work_y {}", work_x, work_y);

    for y in 0..WORK_TILE_SIZE {
      for x in 0..WORK_TILE_SIZE {
        d.push(Window::new(
          x * work_x,
          (x + 1) * work_x,
          y * work_y,
          (y + 1) * work_y,
        ));
      }
    }

    println!("Work packages {}", d.len());
    Arc::new(Mutex::new(d))
  };

  let (tx, rx) = mpsc::channel();

  let mut threads = Vec::new();

  let tmr = BasicTimer::new();

  for i in 0..THREAD_COUNT {
    let work_packages = domains.clone();
    let tx = tx.clone();
    let world = world.clone();

    let thread = thread::spawn(move || loop {
      let mut rng = thread_rng();

      let current_work_package = {
        let mut work_queue = work_packages.lock().unwrap();
        work_queue.pop()
      };

      if !current_work_package.is_some() {
        println!("Thread {} out of work, shutting down", i);
        break;
      }

      let current_work_package = current_work_package.unwrap();

      let mut pixels = Vec::new();

      for y in current_work_package.ymin..current_work_package.ymax {
        for x in current_work_package.xmin..current_work_package.xmax {
          let mut col = Vec3::same(0f32);

          for _ in 0..RAYS_PER_PIXEL {
            let dx: f32 = rng.gen();
            let u = (x as f32 + dx) / nx as f32;

            let dy: f32 = rng.gen();
            let v = (y as f32 + dy) / ny as f32;

            let r = cam.ray_at(u, v);
            col += color(&r, &world, 0);
          }

          col /= RAYS_PER_PIXEL as f32;
          col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
          let pixel_color = to_rgb8(col);
          pixels.push(pixel_color);
        }
      }

      tx.send((i, current_work_package, pixels)).unwrap();
    });
    threads.push(thread);
  }

  for t in threads {
    t.join().unwrap();
  }

  tmr.end();
  println!(
    "Raytraced using {} threads, total time {} seconds",
    THREAD_COUNT,
    tmr.elapsed_seconds()
  );

  std::mem::drop(tx);

  let mut image_pixels = Vec::new();
  image_pixels.resize((nx * ny) as usize, RGB8::new(0, 0, 0));

  for (tid, wpkg, pixels) in rx {
    println!("Merging work package {:?} from thread {}", wpkg, tid);

    // let fname = format!(
    //     "wk_{}_{}_{}_{}.png",
    //     wpkg.xmin, wpkg.xmax, wpkg.ymin, wpkg.ymax
    // );

    // write_image(&fname, wpkg.width() as u32, wpkg.height() as u32, &pixels)
    //     .expect("Failed to write file!");

    let mut idx = 0;
    for y in wpkg.ymin..wpkg.ymax {
      for x in wpkg.xmin..wpkg.xmax {
        image_pixels[((ny - y - 1) * nx + x) as usize] = pixels[idx];
        idx += 1;
      }
    }
  }

  write_image("raytraced.png", nx as u32, ny as u32, &image_pixels)
    .expect("Failed to write image!");
}
