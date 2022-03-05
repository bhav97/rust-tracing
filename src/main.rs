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

use scene::material::dielectric::Dielectric;
use scene::material::matte::Matte;
use scene::material::metal::Metal;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let world = setup_world();
    let camera = Camera::default();

    let no_preview = true;

    if no_preview {

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
    
        return Ok(());
    }

    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            400,
            (400 as f64 / camera.aspect_ratio) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            400,
            (400.0 / camera.aspect_ratio) as u32,
        )
        .map_err(|e| e.to_string())?;
    // Create a red-green gradient

    canvas.clear();
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(0, 0, 400, (400.0 / camera.aspect_ratio) as u32)),
    )?;
    // canvas.copy_ex(
    //     &texture,
    //     None,
    //     Some(Rect::new(450, 100, 256, 256)),
    //     0.0,
    //     None,
    //     false,
    //     false,
    // )?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut y = 0;
    let img_height = (400.0 / camera.aspect_ratio) as u32;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                        println!("P3");
                        println!("{} {}", 400, y);
                        println!("255");
                        for x in 0..(y as usize) {
                        for i in 0..400 {
                            println!("{} {} {}", buffer[x + i*3], buffer[x + i*3+1], buffer[x + i*3+2]);
                        }
                        }
                    })?;
                }
                _ => {}
            }
        }

        if y >= img_height {
            continue;
        }

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for x in 0..(pitch / 3) {
                let col =
                    world.render_pixel(&camera, (x as u32, img_height - y), (400, img_height));
                let offset = y as usize * pitch + x * 3;
                if offset >= 270000 {
                    println!("{} {}", y, img_height);
                }
                buffer[offset] = (255.999 * col.x) as u8;
                buffer[offset + 1] = (255.999 * col.y) as u8;
                buffer[offset + 2] = (255.999 * col.z) as u8;
            }
        })?;
        y += 1;
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}

fn setup_world() -> World {
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
        Box::new(Dielectric::new(RgbColor::new(0.9f64, 0.9f64, 0.9f64), 1f64)),
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

    return world;

    // Image
    // let aspect_ratio = 16f64/9f64;
    // let image_width = 400;
    // let image_height = (image_width as f64 / aspect_ratio) as u32;

    // create a camera at 0,0,0 with a 16:9 aspect ratio and focal length of 1u
    let camera = Camera::default();
    eprintln!("{:#?}", camera);

}
