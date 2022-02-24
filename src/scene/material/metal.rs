use crate::scene::material::Material;
use crate::color::rgb::RgbColor;
use crate::geometry::vec3::Vector;
use crate::geometry::vec3::Vector as Point;
use crate::scene::intersect::Intersection;
use crate::scene::ray::Ray;

pub struct Metal {
    albedo: RgbColor,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: RgbColor, fuzz: f64) -> Self {
        if fuzz > 1f64 {
            Metal { albedo, fuzz: 1f64 }
        } else {
            Metal { albedo, fuzz }
        }
    }

    fn reflect(&self, hit: &Point, incident: &Vector, normal: &Vector) -> Ray {
        let mut reflection = *incident - *normal * Vector::dot(*incident, *normal) * 2f64;
        reflection = reflection + self.random_in_unit_sphere()*self.fuzz;
        Ray::new(*hit, reflection)
    }
}

impl Material for Metal {
    type Albedo = RgbColor;
    fn scatter(&self, hit_ray: &Ray, hit: &Intersection) -> Option<Ray> {
        Some(self.reflect(&hit.point, &hit_ray.direction, &hit.normal))
    }

    fn albedo(&self) -> &Self::Albedo {
        &self.albedo
    }
}