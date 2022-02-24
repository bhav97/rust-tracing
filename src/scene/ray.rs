use crate::geometry::vec3::Vector;
pub use crate::geometry::vec3::Vector as Point;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin,
            direction: Vector::unit(direction)
        }
    }

    pub fn default(direction: Vector) -> Ray {
        Ray {
            origin: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64
            },
            direction: Vector::unit(direction)
        }
    }

    pub fn at(&self, time: f64) -> Point {
        self.origin + self.direction*time
    }
}