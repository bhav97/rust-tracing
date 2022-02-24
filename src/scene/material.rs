use crate::color::rgb::RgbColor;
use crate::geometry::vec3::Vector;
use crate::scene::intersect::Intersection;
use crate::scene::ray::Ray;
use rand::Rng;

pub mod matte;
pub mod metal;

pub trait Material {
    type Albedo;
    fn scatter(&self, hit_ray: &Ray, hit: &Intersection) -> Option<Ray>;
    fn albedo(&self) -> &Self::Albedo;

    fn random_in_unit_sphere(&self) -> Vector {
        let x: f64 = rand::thread_rng().gen_range(0f64..1f64);
        let y: f64 = rand::thread_rng().gen_range(0f64..1f64);
        let z: f64 = rand::thread_rng().gen_range(0f64..1f64);
        Vector { x, y, z }
    }

    fn random_in_hemisphere(&self, normal: Vector) -> Vector {
        let random_vector = self.random_in_unit_sphere();
        if Vector::dot(random_vector, normal) > 0f64 {
            random_vector
        } else {
            random_vector * -1f64
        }
    }
}
