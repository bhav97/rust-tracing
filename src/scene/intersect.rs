use crate::geometry::vec3::Vector as Point;
use crate::geometry::vec3::Vector;
use crate::scene::ray::Ray;
use crate::scene::material::Material;
use crate::color::rgb::RgbColor;

pub struct Intersection {
    // Intersections happen when a ray crosses the bounds of an object that implements this trait
    // The intersection mathematics will be individual to every object depending on its geometry
    // For rendering we need 3 key pieces of information from every intersection of a ray and the
    // object.
    // TODO: A better name for this struct could be Hit, since the rays also reflect/refract

    // The point in 3d space where this intersection/hit occured
    pub point: Point,

    // The normal vector (unit length) at the intersection (normal to the surface)
    pub normal: Vector,

    // Parameter t of the ray, since a ray is parameterised as r(t) = O + Dt
    // where O is the origin of the ray, D is the direction
    // Think of this as the distance travelled by the ray from its origin
    pub t: f64,
}

impl Intersection {
    pub fn new(point: Point, normal: Vector, t: f64) -> Self {
        Intersection {
            point,
            normal,
            t,
        }
    }
}

pub trait Intersect {
    // type HitList = std::vec::Vec::<crate::scene::intersect::Intersections as Trait>::new();

    fn intersects(self: &Self, ray: &Ray, range: (f64, f64)) -> Option<Intersection>;
    fn material(&self) -> &dyn Material<albedo=RgbColor>;
}