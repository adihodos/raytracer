#![allow(dead_code)]

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
mod diffuse_light;
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
mod xy_rect;
mod xz_rect;
mod yz_rect;

use bvh_node::BvhNode;
use camera::{Camera, CameraParameters};
use checker_texture::CheckerTexture;
use constant_texture::ConstantTexture;
use dielectric::Dielectric;
use diffuse_light::DiffuseLight;
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
use xy_rect::XYRect;
use xz_rect::XZRect;
use yz_rect::YZRect;

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

fn color(r: &Ray, world: &Arc<Hitable>, depth: i32) -> Vec3 {
  if let Some(hit) = world.hit(r, 0.001f32, std::f32::MAX) {
    let emitted = hit.mtl.emitted(hit.u, hit.v, hit.p);
    if depth < 50 {
      if let Some((attn, scattered)) = hit.mtl.scatter(r, &hit) {
        return emitted + attn * color(&scattered, world, depth + 1);
      }
    }

    return emitted;
  }

  //Vec3::same(0_f32)

  let unit_direction = vec3::unit_vector(r.direction);
  let t = 0.5f32 * (unit_direction.y + 1f32);
  (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1f32)
}

struct WorldBuilder {}

impl WorldBuilder {
  fn default_world() -> HitableList {
    let mut world = HitableList::new();

    let noise_tex =
      Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(8_f32))));

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0f32, -1000f32, 0f32),
      1000f32,
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

          world.add_object(Arc::new(Sphere::new(center, 0.2f32, mtl)));
        }
      }
    }

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0f32, 1f32, 0f32),
      1f32,
      Arc::new(Dielectric::new(1.5f32)),
    )));

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(-4f32, 1f32, 0f32),
      1f32,
      Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.4f32, 0.2f32, 0.1f32,
      ))))),
    )));

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(4f32, 1f32, 0f32),
      1f32,
      Arc::new(Metal::new(Vec3::new(0.7f32, 0.6f32, 0.5f32), 0f32)),
    )));

    world
  }

  pub fn random_world_bvh() -> (Arc<Hitable>, CameraParameters) {
    let mut world = WorldBuilder::default_world();
    (
      BvhNode::new(world.as_mut_slice(), 0_f32, 1_f32),
      WorldBuilder::default_camera(),
    )
  }

  fn random_world() -> (Arc<Hitable>, CameraParameters) {
    let world = WorldBuilder::default_world();

    (Arc::new(world), WorldBuilder::default_camera())
  }

  fn two_perlin_spheres() -> (Arc<Hitable>, CameraParameters) {
    let perlin_tex =
      Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4_f32))));
    let mut world = HitableList::new();

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0_f32, -1000_f32, 0_f32),
      1000_f32,
      perlin_tex.clone(),
    )));

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0_f32, 2_f32, 0_f32),
      2_f32,
      perlin_tex.clone(),
    )));

    (Arc::new(world), WorldBuilder::default_camera())
  }

  fn two_spheres() -> (Arc<Hitable>, CameraParameters) {
    let odd =
      Arc::new(ConstantTexture::new(Vec3::new(0.2_f32, 0.3_f32, 0.1_f32)));
    let even =
      Arc::new(ConstantTexture::new(Vec3::new(0.9_f32, 0.9_f32, 0.9_f32)));
    let checker_texture = Arc::new(CheckerTexture::new(odd, even));

    let checker_mtl = Arc::new(Lambertian::new(checker_texture));

    let mut world = HitableList::new();

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0_f32, -10_f32, 0_f32),
      10_f32,
      checker_mtl.clone(),
    )));

    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0_f32, 10_f32, 0_f32),
      10_f32,
      checker_mtl.clone(),
    )));

    (Arc::new(world), WorldBuilder::default_camera())
  }

  fn simple_light() -> (Arc<Hitable>, CameraParameters) {
    let perlin_tex =
      Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4_f32))));
    let light_mtl = Arc::new(DiffuseLight::new(Arc::new(
      ConstantTexture::new(Vec3::same(4_f32)),
    )));

    let mut world = HitableList::new();
    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0_f32, -1000_f32, 0_f32),
      1000_f32,
      perlin_tex.clone(),
    )));
    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0_f32, 2_f32, 0_f32),
      2_f32,
      perlin_tex.clone(),
    )));
    world.add_object(Arc::new(Sphere::new(
      Vec3::new(0_f32, 7_f32, 0_f32),
      2_f32,
      light_mtl.clone(),
    )));
    world.add_object(Arc::new(XYRect::new(
      3_f32,
      5_f32,
      1_f32,
      3_f32,
      -2_f32,
      light_mtl.clone(),
    )));

    (Arc::new(world), WorldBuilder::default_camera())
  }

  fn cornell_box() -> (Arc<Hitable>, CameraParameters) {
    let red = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
      Vec3::new(0.65_f32, 0.05_f32, 0.05_f32),
    ))));

    let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
      Vec3::new(0.73_f32, 0.73_f32, 0.73_f32),
    ))));

    let green = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(
      Vec3::new(0.12_f32, 0.45_f32, 0.15_f32),
    ))));

    let light = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
      Vec3::same(15_f32),
    ))));

    let mut world = HitableList::new();
    world.add_object(Arc::new(YZRect::new(
      0_f32, 555_f32, 0_f32, 555_f32, 555_f32, green,
    )));

    world.add_object(Arc::new(YZRect::new(
      0_f32, 555_f32, 0_f32, 555_f32, 0_f32, red,
    )));

    world.add_object(Arc::new(XZRect::new(
      213_f32, 343_f32, 227_f32, 332_f32, 554_f32, light,
    )));

    world.add_object(Arc::new(XZRect::new(
      0_f32,
      555_f32,
      0_f32,
      555_f32,
      0_f32,
      white.clone(),
    )));

    world.add_object(Arc::new(XZRect::new(
      0_f32,
      555_f32,
      0_f32,
      555_f32,
      555_f32,
      white.clone(),
    )));

    let cam_params = {
      let mut cp = CameraParameters::default();
      cp.lookfrom = Vec3::new(278_f32, 278f32, -800_f32);
      cp.lookat = Vec3::new(278_f32, 278_f32, 0_f32);
      cp.world_up = Vec3::new(0_f32, 1_f32, 0_f32);
      cp.focus_dist = 10_f32;
      cp.aperture = 0_f32;
      cp.field_of_view = 40_f32;
      cp.time0 = 0_f32;
      cp.time1 = 1_f32;

      cp
    };

    (Arc::new(world), cam_params)
  }

  fn default_camera() -> CameraParameters {
    let mut defparams = CameraParameters::default();

    defparams.lookfrom = Vec3::new(13f32, 2f32, 3f32);
    defparams.lookat = Vec3::new(0f32, 0f32, 0f32);
    defparams.world_up = Vec3::new(0_f32, 1_f32, 0_f32);
    defparams.focus_dist = 10f32;
    defparams.aperture = 0.1f32;
    defparams.field_of_view = 20_f32;
    defparams.time0 = 0_f32;
    defparams.time1 = 1_f32;

    defparams
  }
}

const THREAD_COUNT: i32 = 4;
const WORK_TILE_SIZE: u32 = 4;

fn main() {
  let nx = 1200;
  let ny = 800;
  const RAYS_PER_PIXEL: u32 = 128;

  let window = Window::new(0, nx, 0, ny);

  let (world, cam_params) =
        //WorldBuilder::two_perlin_spheres();
//            WorldBuilder::random_world();
  //          WorldBuilder::two_spheres();
          WorldBuilder::random_world_bvh();
  //            WorldBuilder::simple_light();
  //    WorldBuilder::cornell_box();

  let cam = Camera::new(
    cam_params.lookfrom,
    cam_params.lookat,
    cam_params.world_up,
    cam_params.field_of_view,
    nx as f32 / ny as f32,
    cam_params.aperture,
    cam_params.focus_dist,
    cam_params.time0,
    cam_params.time1,
  );

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
