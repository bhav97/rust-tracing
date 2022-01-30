// use geometry::vec3;
mod geometry;
mod color;
mod scene;
mod world;

// use crate::geometry::vec3;
use crate::geometry::vec3;
use color::rgb;
use scene::ray;
use scene::camera::Camera;
use world::World;
use geometry::sphere::Sphere;
use geometry::vec3::Vector as Point;

use color::filter::Filter;
use color::antialias::AntialiasingFilter;
use color::gamma::GammaCorrection;

pub fn ray_color(r: ray::Ray) -> rgb::RgbColor {
    // if ()
    let t = hit_sphere(&r);
    if t > 0f64 {
        let normal = vec3::Vector::unit(r.at(t) - vec3::Vector::new(0f64, 0f64, -1f64));
        return rgb::RgbColor::new(normal.x + 1f64, normal.y + 1f64, normal.z + 1f64)*0.5;
    }
    // rgb::RgbColor::new(255f64, 255f64, 255f64)
    let dir_unit = vec3::Vector::unit(r.direction);
    let t = 0.5*(dir_unit.y + 1.0f64);
    rgb::RgbColor::new(1f64, 1f64, 1f64)*(1f64 - t) + rgb::RgbColor::new(0.5f64, 0.7f64, 1.0f64)*t
}

pub fn hit_sphere(r: &ray::Ray) -> f64 {
    let oc = r.origin - vec3::Vector::new(0f64, 0f64, -1f64);
    let a = vec3::Vector::dot(r.direction, r.direction);
    let b = vec3::Vector::dot(oc, r.direction)*2.0f64;
    let c = vec3::Vector::dot(oc, oc) - 0.5*0.5;
    let dt = b*b - a*c*4f64;
    if dt < 0f64 {
        -1f64
    } else {
        (-b - f64::sqrt(dt))/ a*2.0f64
    }
}

fn main() {
    // let vec = vec3::Vector::default();
    // println!("{:?}", vec);
    // sample_gen(256, 256);
    // Create our virtual world
    let mut world = World::new();
    let ball = Sphere::new(Point::new(0f64, 0f64, -1f64), 0.5f64);
    let ball2 = Sphere::new(Point::new(1f64, 1f64, -3f64), 1f64);
    let ground = Sphere::new(Point::new(0f64, -100.5f64, -1f64), 100f64);
    world.add(Box::new(ground));
    world.add(Box::new(ball));
    world.add(Box::new(ball2));

    // Image 
    // let aspect_ratio = 16f64/9f64;
    // let image_width = 400;
    // let image_height = (image_width as f64 / aspect_ratio) as u32;

    // create a camera at 0,0,0 with a 16:9 aspect ratio and focal length of 1u
    let camera = Camera::default();
    eprintln!("{:#?}", camera);

    // render(&camera, (400f64 / camera.aspect_ratio) as u32, 400);
    let mut image = world.render(&camera, 400);
    // Gamma correction 0->1 brighten image, gamma compression
    // Gamma correction > 1 -> darken image, gamma expansion
    // hopefully this oversimplified implementation is correct
    let filter = GammaCorrection::new(0.5f64);

    filter.apply_filter(&mut image);
    // generate PPM
    // TODO: there must be better outputs
    
    println!("P3");
    println!("{} {}", 400, (400f64 / camera.aspect_ratio) as u32);
    println!("255");

    for pixel in image {
        println!("{}", pixel);
    }
}

fn render(camera: &Camera, height: u32, width: u32) {
    // Camera

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for y in  (0..height).rev() {
        eprint!("Scanlines remaining {:03}\r", y);
        for x in 0..width {
            // let pixel = rgb::RgbColor::default();
            let u = x as f64 / (width - 1) as f64;
            let v = y as f64 / (height - 1) as f64;
            let r = camera.get_ray(u, v);
            println!("{}", ray_color(r));
        }
    }

    eprintln!("\nDone");
}