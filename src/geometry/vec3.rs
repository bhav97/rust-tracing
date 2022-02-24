use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn default() -> Self {
        Vector {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    // Vector cross product
    pub fn cross(v1: Vector, v2: Vector) -> Self {
        Vector {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    pub fn dot(v1: Vector, v2: Vector) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn len(self) -> f64 {
        f64::sqrt(self.len_sq())
    }

    pub fn len_sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dir(self) -> Self {
        Vector::unit(self)
    }

    pub fn unit(v: Vector) -> Vector {
        v / v.len()
    }
}

// Operator Trait implementations follow

// Vector addition
impl Add for Vector {
    type Output = Self;

    fn add(self, v3: Vector) -> Self::Output {
        Vector {
            x: self.x + v3.x,
            y: self.y + v3.y,
            z: self.z + v3.z,
        }
    }
}

// Vector subtraction
impl Sub for Vector {
    type Output = Self;

    fn sub(self, v3: Vector) -> Self::Output {
        Vector {
            x: self.x - v3.x,
            y: self.y - v3.y,
            z: self.z - v3.z,
        }
    }
}

// Weird Vector multiplication
impl Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, v: Vector) -> Self::Output {
        Self {
            x: v.x * self.x,
            y: v.y * self.y,
            z: v.z * self.z,
        }
    }
}

// Vector scalar multiplication
impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, s: f64) -> Self::Output {
        Self {
            x: s * self.x,
            y: s * self.y,
            z: s * self.z,
        }
    }
}

// Vector scalar division
impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, s: f64) -> Self::Output {
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

// Module tests follow
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        let v1 = Vector::new(1f64, 0f64, -1f64);
        let v2 = Vector::new(1f64, 2f64, 3f64);
        // i - k + i + 2j + 3k = 2i + 2j + 2k
        assert_eq!(Vector::new(2f64, 2f64, 2f64), v1 + v2);
        assert_eq!(Vector::new(2f64, 2f64, 2f64), v2 + v1);
        // i + 2j + 3k - i + k = 2i + 2j + 2k
        assert_eq!(Vector::new(0f64, 2f64, 4f64), v2 - v1);
        // i - k - i - 2j - 3k = 2i + 2j + 2k
        assert_eq!(Vector::new(0f64, -2f64, -4f64), v1 - v2);
    }

    #[test]
    fn test_vec_len() {
        let v1 = Vector::new(4f64, 0f64, 3f64);
        let v2 = Vector::new(6f64, 8f64, 0f64);
        assert_eq!(v1.len(), 5f64);
        assert_eq!(v2.len(), 10f64);
    }

    #[test]
    fn test_vec_len_sq() {
        let v1 = Vector::new(4f64, 0f64, 3f64);
        let v2 = Vector::new(6f64, 8f64, 0f64);
        assert_eq!(v1.len_sq(), 25f64);
        assert_eq!(v2.len_sq(), 100f64);
        assert_eq!((v1 + v2).len_sq(), 173f64)
    }

    #[test]
    fn test_vec_cross() {
        let v1 = Vector::new(1f64, 1f64, 1f64);
        let v2 = Vector::new(1f64, 0f64, 1f64);
        let v3 = Vector::new(1f64, 2f64, 1f64);
        let v4 = Vector::new(-1f64, 1f64, -1f64);

        // (i + j + k) x (i + k) = i - k
        assert_eq!(Vector::new(1f64, 0f64, -1f64), Vector::cross(v1, v2));
        // (i + k) x (i + j + k)
        assert_eq!(Vector::new(-1f64, 0f64, 1f64), Vector::cross(v2, v1));

        // (i + k) x (i + k) = 0
        assert_eq!(Vector::new(0f64, 0f64, 0f64), Vector::cross(v2, v2));
        // (i + 2j + k) x (j - i - k) = 3k - 3i
        assert_eq!(Vector::new(-3f64, 0f64, 3f64), Vector::cross(v3, v4));
        // (j - i - k) x (i + 2j + k) = 3i - 3k
        assert_eq!(Vector::new(3f64, 0f64, -3f64), Vector::cross(v4, v3));

        // (j - i - k) x (i + k) = i - k
        assert_eq!(Vector::new(1f64, 0f64, -1f64), Vector::cross(v4, v2));
    }

    #[test]
    fn test_vec_dot() {
        let v1 = Vector::new(1f64, 2f64, 1f64);
        let v2 = Vector::new(-1f64, 1f64, -1f64);
        assert_eq!(6f64, Vector::dot(v1, v1));
        assert_eq!(0f64, Vector::dot(v2, v1));
        assert_eq!(3f64, Vector::dot(v2, v2));
    }

    #[test]
    fn test_unit_of_vector() {
        let mut v = Vector::new(1f64, 1f64, 1f64);

        assert_eq!(
            Vector::new(
                1f64 / f64::sqrt(3f64),
                1f64 / f64::sqrt(3f64),
                1f64 / f64::sqrt(3f64)
            ),
            Vector::unit(v)
        );

        // This might fail due to floating point inaccuracies on some numbers
        v = v * 4f64;
        assert_eq!(
            Vector::new(
                1f64 / f64::sqrt(3f64),
                1f64 / f64::sqrt(3f64),
                1f64 / f64::sqrt(3f64)
            ),
            Vector::unit(v)
        );
    }
}
