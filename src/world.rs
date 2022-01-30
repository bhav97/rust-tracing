use crate::color::rgb::RgbColor;
use crate::geometry::vec3::Vector;
use crate::geometry::vec3::Vector as Point;
use crate::scene::camera::Camera;
use crate::scene::intersect::Intersect;
use crate::scene::intersect::Intersection;
use crate::scene::ray::Ray;
use std::vec::Vec;
use crate::scene::material::Material;
use crate::scene::material::matte::Matte;

/// A virtual world is represented here
pub struct World {
    contents: Vec<Box<dyn Intersect>>,
}

impl World {
    const MAX_DEPTH: usize = 50;
    const SAMPLES_PER_PIXEL: usize = 100;
    /// Returns a new empty world
    ///
    /// # Examples
    /// ```
    /// use world::World;
    /// let world = World::new();
    /// ```
    pub fn new() -> Self {
        World {
            contents: Vec::new(),
        }
    }

    pub fn size(self) -> usize {
        self.contents.len()
    }

    pub fn render(&self, camera: &Camera, img_width: u32) -> Vec<RgbColor> {
        let mut render = Vec::<RgbColor>::new();
        let img_height = (img_width as f64 / camera.aspect_ratio) as u32;
        // TODO: render in multiple threads
        for y in (0..img_height).rev() {
            for x in 0..img_width {
                let u = x as f64 / (img_width - 1) as f64;
                let v = y as f64 / (img_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                render.push(self.raytrace(&ray, World::MAX_DEPTH));
            }
        }

        return render;
    }

    pub fn add(&mut self, object: Box<dyn Intersect>) {
        self.contents.push(object);
    }

    fn raytrace(&self, ray: &Ray, depth: usize) -> RgbColor {
        // Lose all energy after max hits
        if depth <= 0 {
            return RgbColor::default();
        }

        if let Some((ray_hit, material)) = self.hit(&ray, (0.0001f64, f64::INFINITY)) {
            let target = ray_hit.point + ray_hit.normal;
            
            // 3. Recursive ray bounces integrated with material scattering
            if let Some(scattered_ray) = material.scatter(ray, &ray_hit) {
                return self.raytrace(&scattered_ray, depth - 1) * *material.albedo()
            } else {
                return RgbColor::default();
            }

            // 2. Recursive ray bounces
            // return self.raytrace(&Ray::new(ray_hit.point, target - ray_hit.point), depth - 1) * 0.5f64;

            // 1. Color the first intersection
            // return ray_hit.normal * 0.5f64;
        } else {
            // Nothing is hit, generate gradient background
            let t: f64 = 0.5 * ray.direction.y + 1f64;
            return RgbColor::new(1f64, 1f64, 1f64) * (1f64 - t)
                + RgbColor::new(0.5f64, 0.7f64, 1f64) * t;
            // println!("{}",
            //    RgbColor::new(1f64, 1f64, 1f64) * (1f64-t) +
            //    RgbColor::new(0.5f64, 0.7f64, 1f64) * t
            // );

            // or just black
            // render.push(RgbColor::default());
        }
    }

    // private method
    fn hit(&self, ray: &Ray, range: (f64, f64)) -> Option<(Intersection, &dyn Material<albedo = RgbColor>)> {
        let mut closest_intersect =
            Intersection::new(Point::default(), Vector::default(), range.1);
        let mut did_it_intersect = false;
        let mut material_hit: Option<&dyn Material<albedo = RgbColor>> = None;

        // For every object in the world i.e. our content vector
        for object in &self.contents {
            // Check if the ray and object intersect
            let object_intersect_range = (range.0, closest_intersect.t);
            if let Some(intr) = object.intersects(ray, object_intersect_range) {
                // update closest intersection to find any further
                // intersections between range.0 and the current intersection
                closest_intersect = intr;
                material_hit = Some(object.material());
            }
        }

        if let Some(material) = material_hit {
            return Some((closest_intersect, material));
        }

        None
    }
}
