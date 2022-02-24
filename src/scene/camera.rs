use crate::geometry::vec3::Vector;
use crate::geometry::vec3::Vector as Point;
use crate::scene::ray::Ray;

#[derive(Debug, PartialEq)]
pub struct Camera {
    pub aspect_ratio: f64,
    origin: Point,
    bottom_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    // The camera struct consists of 2 parts
    // the actual eye of the camera at position `pos` and the viewport or the canvas `focal_length` away from `pos`
    //                          |
    // []< ---------------------|
    //                          |
    // pos ---focal_length--- viewport
    pub fn new(pos: Point, aspect_ratio: f64, focal_length: f64) -> Self {
        let viewport_height = 2;
        let viewport_width = aspect_ratio * viewport_height as f64;

        let origin = pos;
        let horizontal = Vector::new(viewport_width, 0f64, 0f64);
        let vertical = Vector::new(0f64, viewport_height as f64, 0f64);
        let bottom_left_corner =
            origin - horizontal / 2f64 - vertical / 2f64 - Vector::new(0f64, 0f64, focal_length);
        Camera {
            aspect_ratio,
            origin,
            bottom_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn default() -> Self {
        Camera::new(Point::new(0f64, 0f64, 0f64), 16f64 / 9f64, 1f64)
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_new() {
        assert_eq!(
            Camera::default(),
            Camera {
                origin: Point::new(0f64, 0f64, 0f64),
                bottom_left_corner: Vector::new(-16 as f64 / 9 as f64, -1f64, -1f64),
                horizontal: Vector::new(2f64 * 16 as f64 / 9 as f64, 0f64, 0f64),
                vertical: Vector::new(0f64, 2f64, 0f64)
            }
        );
    }
}
