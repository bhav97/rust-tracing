use crate::color::rgb::RgbColor;
use crate::geometry::vec3::Vector;
use crate::geometry::vec3::Vector as Point;
use crate::scene::camera::Camera;
use crate::scene::intersect::Intersect;
use crate::scene::intersect::Intersection;
use crate::scene::material::Material;
use crate::scene::ray::Ray;
use rand::Rng;
use std::vec::Vec;

/// A virtual world is represented here
pub struct World {
    contents: Vec<Box<dyn Intersect>>,
}

impl World {
    const MAX_DEPTH: usize = 50;
    const SAMPLES_PER_PIXEL: usize = 50;
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

    pub fn render_pixel(&self, camera: &Camera, coords: (u32, u32), dims: (u32, u32)) -> RgbColor {
        let mut pixel_color = RgbColor::default();
        let mut rand = rand::thread_rng();
        for _ in 0..Self::SAMPLES_PER_PIXEL {
            let u = coords.0 as f64 / ((dims.0 - 1) as f64 + rand.gen_range(0f64..1f64));
            let v = coords.1 as f64 / ((dims.1 - 1) as f64 + rand.gen_range(0f64..1f64));
            let ray = camera.get_ray(u, v);
            pixel_color = pixel_color + self.raytrace(&ray, World::MAX_DEPTH);
        }
        pixel_color = pixel_color * (1f64 / World::SAMPLES_PER_PIXEL as f64);
        pixel_color
    }

    /// Returns the rendering of the world from the input camera viewport as a vector of RgbColor. The render dimension
    /// is controlled by the input image width and the camera aspenct ratio. 
    /// 
    /// Generates a ray from the each of the pixels of the viewport and traces it in the world. For a given pixel,
    /// `World::SAMPLES_PER_PIXEL` samples are generated. The color of the rays is then averaged for anti-aliasing.
    /// 
    /// # Arguments
    /// * `camera` - Ref to a camera object which controls the viewport settings
    /// * `img_width` - Width of the render
    /// 
    /// # Returns
    /// * Vec<RgbColor> - the rendered scene 
    /// 
    /// # Examples
    /// ```
    /// const IMG_WIDTH = 800;
    /// let mut world = World::new();
    /// let sphere1 = Sphere::new(
    ///     Point::new(0.0, 0.0, -1.0),
    ///     0.5,
    ///     Box::new(Matte::new(RgbColor::new(0.8, 0.8, 0.8)))
    /// );
    /// world.add(Box::new(sphere1));
    /// let camera = Camera::default();
    /// let rendered_scene = world.render(&camera, IMG_WIDTH);
    /// ```
    pub fn render(&self, camera: &Camera, img_width: u32) -> Vec<RgbColor> {
        // 1. Calculate image height from width using the camera aspect ratio
        let img_height = (img_width as f64 / camera.aspect_ratio) as u32;
        // 2. Initialize with capacity because it's known, and avoid reallocation
        let mut render = Vec::<RgbColor>::with_capacity((img_height * img_width) as usize);

        // TODO: render in multiple threads
        for y in (0..img_height).rev() {
            eprint!("Generating line {:#3?}\r", img_height - y);
            for x in 0..img_width {
                // Oversample and average with jitter > antialiasing
                let mut pixel_color = RgbColor::default();
                let mut rand = rand::thread_rng();
                for _ in 0..World::SAMPLES_PER_PIXEL {
                    let u = x as f64 / ((img_width - 1) as f64 + rand.gen_range(0f64..1f64));
                    let v = y as f64 / ((img_height - 1) as f64 + rand.gen_range(0f64..1f64));
                    let ray = camera.get_ray(u, v);
                    pixel_color = pixel_color + self.raytrace(&ray, World::MAX_DEPTH);
                }
                pixel_color = pixel_color * (1f64 / World::SAMPLES_PER_PIXEL as f64);
                render.push(pixel_color);
            }
        }

        return render;
    }

    /// Adds an object implementng the Intersect trait to the world. The stored object is used for calculating 
    /// intersections during ray tracing
    /// 
    /// # Arguments
    /// * `object` - The object 
    /// 
    /// # Examples
    /// ```
    /// let mut world = World::new();
    /// let sphere1 = Sphere::new(
    ///     Point::new(0.0, 0.0, -1.0),
    ///     0.5,
    ///     Box::new(Matte::new(RgbColor::new(0.8, 0.8, 0.8)))
    /// );
    /// world.add(Box::new(sphere1));
    /// ```
    pub fn add(&mut self, object: Box<dyn Intersect>) {
        // TODO: add a remove method
        self.contents.push(object);
    }

    /// Returns the color to be rendered for an input ray in according to the scene setup by the world contents.
    /// 
    /// Recursive function call which will trace the input ray in the scene its children until either the `depth`
    /// is reached or the material absorbs the ray. 
    /// 
    /// If an object from the `World::content` vector is intersected by the ray, the scattered ray is calculated
    /// by the `scatter` function of the `Material` trait.
    /// 
    /// If nothing is hit it a gradient is generated along the y-axis.
    /// 
    /// # Arguments
    /// 
    /// * `ray` - Ref to the ray for which the color has to be computed
    /// * `depth` - the recursion depth for limiting the number of child ray calculations
    /// 
    /// # Returns
    /// 
    /// * RgbColor - color to be rendered for the input ray
    /// 
    fn raytrace(&self, ray: &Ray, depth: usize) -> RgbColor {
        // 1. Lose all energy after max hits
        if depth <= 0 {
            return RgbColor::default();
        }

        // 2. Check if the input ray intersects an object in the world
        if let Some((ray_hit, material)) = self.hit(&ray, (0.01, f64::INFINITY)) {
            // 3. child ray bounces with material scattering
            if let Some(child_ray) = material.scatter(ray, &ray_hit) {
                return self.raytrace(&child_ray, depth - 1) * *material.albedo();
            } else {
                // 4. If the scatter function does not return a child ray, the incident ray has been absorbed
                return RgbColor::default();
            }
        } else {
            // 5. Nothing is hit, generate gradient background
            let t: f64 = 0.5 * ray.direction.y + 1f64;
            return RgbColor::new(1f64, 1f64, 1f64) * (1f64 - t)
                + RgbColor::new(0.5f64, 0.7f64, 1f64) * t;
        }
    }

    /// Iterates over the content of the world to check whether the input ray intersects with the world content
    /// within the given range and returns the intersection and material hit.
    ///
    /// Since a ray in the world is simply a point (the source) and a vector (the direction), the function needs
    /// a range input within which the intersects function will solve the geometry.
    ///
    /// # Arguments
    ///
    /// * `ray` - Ref to the input ray to check for intersection with the world content
    /// * `range` - The range within which the search for intersection will happen.
    ///
    /// # Optionally returns (when there is an intersection with the world content)
    ///
    /// * `Intersection` - Information about the intersection
    /// * `&dyn Material<Albedo=RgbColor> - The material at intersection`
    ///
    fn hit(
        &self,
        ray: &Ray,
        range: (f64, f64),
    ) -> Option<(Intersection, &dyn Material<Albedo = RgbColor>)> {
        // 1. Initialize closest intersection to max of the input range
        let mut closest_intersect = Intersection::new(Point::default(), Vector::default(), range.1);
        // 2. Initialize the material hit to None
        let mut material_hit: Option<&dyn Material<Albedo = RgbColor>> = None;

        // For every object in the world i.e. our content vector
        for object in &self.contents {
            // 3. Check if the ray and object intersect
            let object_intersect_range = (range.0, closest_intersect.t);
            if let Some(intr) = object.intersects(ray, object_intersect_range) {
                // 4. update closest intersection to find intersections between range.0 and the current intersection
                closest_intersect = intr;
                material_hit = Some(object.material());
            }
        }

        if let Some(material) = material_hit {
            // 4. return the closest intersection info and material for scatter
            return Some((closest_intersect, material));
        }

        // 5. We did not hit anything in our world
        None
    }
}
