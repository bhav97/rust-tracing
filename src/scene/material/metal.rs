use crate::scene::material::Material;
use crate::color::rgb::RgbColor;
use crate::geometry::vec3::Vector;
use crate::geometry::vec3::Vector as Point;
use crate::scene::intersect::Intersection;
use crate::scene::ray::Ray;

pub struct Metal {
    pub albedo: RgbColor,
}

impl Metal {
    fn reflect(&self, hit: &Point, incident: &Vector, normal: &Vector) -> Ray {
        Ray::new(*hit, *incident - *normal * Vector::dot(*incident, *normal) *2f64)
    }
}

impl Material for Metal {
    type albedo = RgbColor;
    fn scatter(&self, hit_ray: &Ray, hit: &Intersection) -> Option<Ray> {
        Some(self.reflect(&hit.point, &hit_ray.direction, &hit.normal))
    }

    fn albedo(&self) -> &Self::albedo {
        &self.albedo
    }
}