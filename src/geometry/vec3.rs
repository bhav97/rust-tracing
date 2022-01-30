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
        let v1 = Vector {
            x: 1f64,
            y: 0f64,
            z: -1f64,
        };
        let v2 = Vector {
            x: 1f64,
            y: 2f64,
            z: 3f64,
        };
        assert_eq!(
            Vector {
                x: 2f64,
                y: 2f64,
                z: 2f64
            },
            v1 + v2
        );
    }

    #[test]
    fn test_vec3_new() {
        let v = Vector::new(1f64, 2f64, 3f64);
        assert_eq!(
            Vector {
                x: 1f64,
                y: 2f64,
                z: 3f64
            },
            v
        );
    }

    #[test]
    fn test_vec_len() {
        let v = Vector {
            x: 4f64,
            y: 0f64,
            z: 3f64,
        };
        assert_eq!(v.len(), 5f64);
    }

    #[test]
    fn test_vec_len_sq() {
        let v = Vector {
            x: 4f64,
            y: 0f64,
            z: 3f64,
        };
        assert_eq!(v.len_sq(), 25f64);
    }

    #[test]
    fn test_vec_cross_1() {
        let v1 = Vector {
            x: 1f64,
            y: 1f64,
            z: 1f64,
        };
        let v2 = Vector {
            x: 1f64,
            y: 0f64,
            z: 1f64,
        };
        assert_eq!(
            Vector {
                x: 1f64,
                y: 0f64,
                z: -1f64
            },
            Vector::cross(v1, v2)
        );
    }

    #[test]
    fn test_vec_cross_self() {
        let v1 = Vector {
            x: 1f64,
            y: 1f64,
            z: 1f64,
        };
        assert_eq!(
            Vector {
                x: 0f64,
                y: 0f64,
                z: 0f64
            },
            Vector::cross(v1, v1)
        );
    }

    #[test]
    fn test_vec_cross_2() {
        let v1 = Vector {
            x: 1f64,
            y: 2f64,
            z: 1f64,
        };
        let v2 = Vector {
            x: -1f64,
            y: 1f64,
            z: -1f64,
        };
        assert_eq!(
            Vector {
                x: -3f64,
                y: 0f64,
                z: 3f64
            },
            Vector::cross(v1, v2)
        );
    }

    #[test]
    fn test_vec_dot() {
        let v1 = Vector {
            x: 1f64,
            y: 2f64,
            z: 1f64,
        };
        let v2 = Vector {
            x: -1f64,
            y: 1f64,
            z: -1f64,
        };
        assert_eq!(
            Vector {
                x: -1f64,
                y: 2f64,
                z: -1f64
            },
            Vector::cross(v1, v2)
        );
    }

    #[test]
    fn test_unit_of_vector() {
        let v = Vector {
            x: -1f64,
            y: 1f64,
            z: -1f64,
        };
        assert_eq!(
            Vector {
                x: -1f64 / f64::sqrt(3f64),
                y: 1f64 / f64::sqrt(3f64),
                z: -1f64 / f64::sqrt(3f64)
            },
            Vector::unit(v)
        );

    }
}
