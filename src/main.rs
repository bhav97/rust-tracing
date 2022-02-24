// use geometry::vec3;
mod color;
mod geometry;
mod scene;
mod world;

// use crate::geometry::vec3;
use crate::geometry::vec3;
use color::rgb;
use geometry::sphere::Sphere;
use geometry::vec3::Vector as Point;
use scene::camera::Camera;
use scene::ray;
use world::World;

use color::antialias::AntialiasingFilter;
use color::filter::Filter;
use color::gamma::GammaCorrection;

use color::rgb::RgbColor;

use scene::material::matte::Matte;
use scene::material::metal::Metal;

fn main() {
    // let vec = vec3::Vector::default();
    // println!("{:?}", vec);
    // sample_gen(256, 256);
    // Create our virtual world
    let mut world = World::new();

    let ball1 = Sphere::new(
        Point::new(0f64, 0f64, -1f64),
        0.5f64,
        Box::new(Matte::new(RgbColor::new(0.8f64, 0.8f64, 0.8f64))),
    );
    let ball2 = Sphere::new(
        Point::new(-1f64, 0f64, -1f64),
        0.5f64,
        Box::new(Metal::new(RgbColor::new(0.8f64, 0.8f64, 0.8f64), 0.5)),
    );
    let ball3 = Sphere::new(
        Point::new(1f64, 0f64, -1f64),
        0.5f64,
        Box::new(Metal::new(RgbColor::new(0.2f64, 0.2f64, 0.2f64), 1f64)),
    );
    // let ball2 = Sphere::new(Point::new(1f64, 1f64, -3f64), 1f64);
    let ground = Sphere::new(
        Point::new(0f64, -100.5f64, -1f64),
        100f64,
        Box::new(Matte::new(RgbColor::new(0.2f64, 0.8f64, 0.8f64))),
    );
    world.add(Box::new(ground));
    world.add(Box::new(ball1));
    world.add(Box::new(ball2));
    world.add(Box::new(ball3));
    // world.add(Box::new(ball2));

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
    // let filter = AntialiasingFilter::new(2);

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
