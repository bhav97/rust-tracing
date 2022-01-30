use crate::geometry::vec3::Vector as Point;
use crate::geometry::vec3::Vector;
use crate::scene::intersect::Intersect;
use crate::scene::intersect::Intersection;
use crate::scene::material::Material;
use crate::scene::ray::Ray;
use crate::color::rgb::RgbColor;
use std::fmt::{Debug, Formatter, Result};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Box<dyn Material<albedo = RgbColor>>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<dyn Material<albedo = RgbColor>>) -> Self {
        Sphere { center, radius, material }
    }

    pub fn default() -> Self {
        use crate::scene::material::matte;

        Sphere {
            center: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: 1f64,
            material: Box::new(matte::Matte { albedo: RgbColor::new(0.8f64, 0.8f64, 0.8f64) })
        }
    }
}

impl Intersect for Sphere {

    fn material(&self) -> &dyn Material<albedo=RgbColor> {
        // time to do some sketchy shit doo-dah doo-dah
        &*self.material
    }

    // Intersection mathemetics
    // A ray in 3d space; r(t) = A + tB, where A is the origin and B is a unit vector
    // A sphere is; x^2 + y^2 + z^2 = r^2, where r is the radius of the sphere and origin is at (0,0,0)
    // the radius of the sphere, r = P - C, where P is any point on the circle an
    // if the ray intersects the sphere, at least one point will satisy both the equations
    // t^2 + 2t(A-C) + (A-C)(A-C) - r^2 = 0
    fn intersects(self: &Self, r: &Ray, range: (f64, f64)) -> Option<Intersection> {
        let oc = r.origin - self.center;
        // ignorimg a because we create rays with unit direction
        // let a = r.direction.len_sq();
        let h_2 = Vector::dot(oc, r.direction);
        let c = oc.len_sq() - self.radius * self.radius;

        let dt = h_2 * h_2 - c;
        if dt < 0f64 {
            return None;
        }

        let sqrtd = f64::sqrt(dt);

        // Find nearest root in the range
        let mut root = -h_2 - sqrtd;
        if root < range.0 || root > range.1 {
            root = -h_2 + sqrtd;
            if root < range.0 || root > range.1 {
                return None;
            }
        }

        // Check if the ray and normal are in the same direction
        let mut normal = (r.at(root) - self.center) / self.radius;
        let front_face: bool = Vector::dot(normal, r.direction) < 0f64; 
        if !front_face {
            normal = normal * -1f64;
        }

        Some(Intersection::new(
            r.at(root),
            normal,
            root,
        ))
    }
}

impl Debug for Sphere {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Sphere of radius {} at {:?}", self.radius, self.center)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_sphere() {
        assert_eq!(
            Sphere::default(),
            Sphere {
                center: Point {
                    x: 0f64,
                    y: 0f64,
                    z: 0f64
                },
                radius: 1f64
            }
        );
    }
}
