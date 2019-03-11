extern crate png;
extern crate rand;
extern crate rgb;

use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::Thread;

use rgb::RGB8;

mod camera;
mod dielectric;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod timer;
mod vec3;
mod window;

use camera::Camera;
use dielectric::Dielectric;
use hitable::*;
use hitable_list::HitableList;
use lambertian::Lambertian;
use material::Material;
use metal::Metal;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use timer::BasicTimer;
use vec3::{random_in_unit_sphere, to_rgb8, Vec3};
use window::Window;

fn write_image(filename: &str, width: u32, height: u32, pixels: &[RGB8]) -> std::io::Result<()> {
    use png::HasParameters;
    use std::fs::File;
    use std::io::BufWriter;

    let file = File::create(filename)?;
    let writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut png_writer = encoder.write_header()?;

    let img_data =
        unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const u8, pixels.len() * 3) };

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

fn random_scene() -> (HitableList, Vec<Rc<Material>>) {
    let mut world = HitableList::new();
    let mut materials: Vec<Rc<Material>> = Vec::new();

    materials.push(Rc::new(Lambertian::new(Vec3::new(0.5f32, 0.5f32, 0.5f32))));

    world.add_object(Box::new(Sphere::new(
        Vec3::new(0f32, -1000f32, 0f32),
        1000f32,
        materials[0].clone(),
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
                let mtl: Rc<Material> = if choose_mat < 0.8f32 {
                    // diffuse
                    Rc::new(Lambertian::new(Vec3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    )))
                } else if choose_mat < 0.95f32 {
                    // metal
                    Rc::new(Metal::new(
                        Vec3::new(
                            0.5f32 * (1f32 + rng.gen::<f32>()),
                            0.5f32 * (1f32 + rng.gen::<f32>()),
                            0.5f32 * (1f32 + rng.gen::<f32>()),
                        ),
                        0.5f32 * rng.gen::<f32>(),
                    ))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5f32))
                };

                materials.push(mtl.clone());
                world.add_object(Box::new(Sphere::new(center, 0.2f32, mtl.clone())));
            }
        }
    }

    let mtl = Rc::new(Dielectric::new(1.5f32));
    materials.push(mtl.clone());
    world.add_object(Box::new(Sphere::new(
        Vec3::new(0f32, 1f32, 0f32),
        1f32,
        mtl.clone(),
    )));

    let mtl = Rc::new(Lambertian::new(Vec3::new(0.4f32, 0.2f32, 0.1f32)));
    materials.push(mtl.clone());
    world.add_object(Box::new(Sphere::new(
        Vec3::new(-4f32, 1f32, 0f32),
        1f32,
        mtl.clone(),
    )));

    let mtl = Rc::new(Metal::new(Vec3::new(0.7f32, 0.6f32, 0.5f32), 0f32));
    materials.push(mtl.clone());
    world.add_object(Box::new(Sphere::new(
        Vec3::new(4f32, 1f32, 0f32),
        1f32,
        mtl.clone(),
    )));

    (world, materials)
}

const THREADS_X: u32 = 4;
const THREADS_Y: u32 = 4;

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10;

    let window = Window::new(0, nx, 0, ny);

    let domains = {
        let mut d = Vec::new();
        let work_x = window.width() / THREADS_X;
        let work_y = window.height() / THREADS_Y;

        for y in 0..THREADS_Y {
            for x in 0..THREADS_X {
                d.push(Window::new(
                    x * work_x,
                    (x + 1) * work_x,
                    y * work_y,
                    (y + 1) * work_y,
                ));
            }
        }

        Arc::new(Mutex::new(d))
    };

    println!("Domains {:?}", domains);

    let (tx, rx) = std::sync::mpsc::channel();

    let mut threads = Vec::new();

    for i in 0..4 {
        let work_packages = domains.clone();
        let tx = tx.clone();

        let thread = std::thread::spawn(move || loop {
            let wkpkg = {
                let mut work = work_packages.lock().unwrap();
                work.pop()
            };

            if !wkpkg.is_some() {
                println!("Thread {} out of work, shutting down", i);
                break;
            }

            let pkg = wkpkg.unwrap();

            //println!("Thread {}, work {:?}", i, pkg);
            tx.send((i, pkg)).unwrap();
        });
        threads.push(thread);
    }

    loop {
        let msg = rx.recv().unwrap();
        println!("Thread {}, work {:?}", msg.0, msg.1);
    }

    for t in threads {
        t.join().unwrap();
    }

    // println!("Domains : {:?}", domains);

    // let mut pixels = Vec::new();
    // let mut rng = thread_rng();

    // let lookfrom = Vec3::new(13f32, 2f32, 3f32);
    // let lookat = Vec3::new(0f32, 0f32, 0f32);
    // let dist_to_focus = 10f32;
    // let aperture = 0.1f32;

    // let cam = Camera::new(
    //     lookfrom,
    //     lookat,
    //     Vec3::new(0f32, 1f32, 0f32),
    //     20f32,
    //     nx as f32 / ny as f32,
    //     aperture,
    //     dist_to_focus,
    // );

    // let (world, materials) = random_scene();

    // let tmr = BasicTimer::new();

    // for y in (window.ymin..window.ymax).rev() {
    //     for x in window.xmin..window.xmax {
    //         let mut col = Vec3::same(0f32);

    //         for _ in 0..ns {
    //             let dx: f32 = rng.gen();
    //             let u = (x as f32 + dx) / nx as f32;

    //             let dy: f32 = rng.gen();
    //             let v = (y as f32 + dy) / ny as f32;

    //             let r = cam.ray_at(u, v);
    //             col += color(&r, &world, 0);
    //         }

    //         col /= ns as f32;
    //         col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
    //         let pixel_color = to_rgb8(col);
    //         pixels.push(pixel_color);
    //     }
    // }

    // tmr.end();

    // println!("Raytraced in {} seconds", tmr.elapsed_seconds());
    // write_image("raytraced.png", nx as u32, ny as u32, &pixels).expect("Failed to write image!");
}
