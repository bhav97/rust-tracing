use crate::scene::material::Material;
use crate::color::rgb::RgbColor;
use crate::geometry::vec3::Vector;
use crate::geometry::vec3::Vector as Point;
use crate::scene::ray::Ray;
use crate::scene::intersect::Intersection;
use rand::Rng;

pub struct Matte {
    pub albedo: RgbColor,
}

impl Matte {
    /// Lamberts cosine law
    /// RgbColor ∝ dot(I,N)
    /// I = I0*Kd*dot(I,N)/(|I|*|N|)
    /// N is the normal of the surface
    /// I is the incident vector
    /// As the angle increases, the light gets weaker
    fn lambertian_diffuse(&self, incident: &Vector, normal: &Vector, hit: &Point) -> Ray {
        let x: f64 = rand::thread_rng().gen_range(0f64..1f64);
        let y: f64 = rand::thread_rng().gen_range(0f64..1f64);
        let z: f64 = rand::thread_rng().gen_range(0f64..1f64);
        let dir = Vector::new(x, y, z) + *normal;
        if dir.len() < 0.001f64 {
            return Ray::new(*hit, *normal);
        }
        Ray::new(*hit, dir)
    }
}

impl Material for Matte {
    type albedo = RgbColor;
    fn scatter(&self, hit_ray: &Ray, hit: &Intersection) -> Option<Ray> {
        Some(self.lambertian_diffuse(&hit_ray.direction, &hit.normal, &hit.point))
    }

    fn albedo(&self) -> &Self::albedo {
        &self.albedo
    }
}