use crate::scene::ray::Ray;
use crate::color::rgb::RgbColor;
use crate::scene::intersect::Intersection;

pub mod metal;
pub mod matte;

pub trait Material {
    type albedo; 
    fn scatter(&self, hit_ray: &Ray, hit: &Intersection) -> Option<Ray>;
    fn albedo(&self) -> &Self::albedo;
}