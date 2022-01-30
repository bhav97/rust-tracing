use crate::color::rgb::RgbColor;
use crate::color::filter::Filter;

pub struct AntialiasingFilter {
    pixels_per_chunk: u32,
    image_width: u32
}

impl AntialiasingFilter {
    pub fn new(pixels_per_chunk: u32) -> Self {
        AntialiasingFilter {
            pixels_per_chunk,
            image_width: 400
        }
    }
}

impl Filter for AntialiasingFilter {
    fn apply_filter(&self, render: &mut Vec<RgbColor>) {
        // render.windows(4)
        // .zip(render)
        // .inspect(|p| eprintln!("{:?}", p))
        // .collect::<Vec<_>>();

        // for chunk in render.windows(self.pixels_per_chunk as usize) {

        // }

        // for pixel in render.step_ {
        //     for iter in 0..self.samples_per_pixel {

        //     }
        // }
    }
}