use crate::scene::material::Material;
use crate::color::rgb::RgbColor;
use crate::geometry::vec3::Vector;
use crate::geometry::vec3::Vector as Point;
use crate::scene::ray::Ray;
use crate::scene::intersect::Intersection;

pub struct Matte {
    albedo: RgbColor,
}

impl Matte {
    pub fn new(albedo: RgbColor) -> Self {
        Matte { albedo }
    }

    /// Lamberts cosine law
    /// RgbColor ∝ dot(I,N)
    /// I = I0*Kd*dot(I,N)/(|I|*|N|)
    /// N is the normal of the surface
    /// I is the incident vector
    /// As the angle increases, the light gets weaker
    fn lambertian_diffuse(&self, incident: &Vector, normal: &Vector, hit: &Point) -> Option<Ray> {
        // let dir = self.random_in_unit_sphere() + *normal;
        let dir = self.random_in_hemisphere(*incident) + *normal;
        if dir.len() < 0.001f64 {
            // return Ray::new(*hit, *normal);
            return None;
        }
        Some(Ray::new(*hit, dir))
    }
}

impl Material for Matte {
    type Albedo = RgbColor;
    fn scatter(&self, hit_ray: &Ray, hit: &Intersection) -> Option<Ray> {
        self.lambertian_diffuse(&hit_ray.direction, &hit.normal, &hit.point)
    }

    fn albedo(&self) -> &Self::Albedo {
        &self.albedo
    }
}