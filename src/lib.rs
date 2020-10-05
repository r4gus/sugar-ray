extern crate num_traits;

use std::ops;
use num_traits::Float;

/** (x, y, z) Coordinate representing a point in 3-dimensional space.
 */
#[derive(PartialEq, Clone, Debug, Copy)]
struct Point<T: Float + Copy> {
    x: T, 
    y: T, 
    z: T, 
}

impl<T: Copy + Float> Point<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/** Get the displacement Q of a point P in the direction of and by magnitude of the vector V = Q - P.
 * 
 * Add a vector V = (x, y, z) to a point P = (x, y, z) to get it's displacement Q.
 *
 */
impl<T: Copy + Float> ops::Add<Vector<T>> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Vector<T>) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/** Subtract a point P from a point Q to get a vector V = Q - P.
 */
impl<T: Copy + Float> ops::Sub<Point<T>> for Point<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Vector<T> {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/** Subtract a vector V from a point Q to get it's origin P.
 *
 * V = Q - P
 * P = Q - V
 */
impl<T: Copy + Float> ops::Sub<Vector<T>> for Point<T> {
    type Output = Self;
    
    fn sub(self, rhs: Vector<T>) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/** Vector representing magnitude and direction in 3-dimensional space.
 */
#[derive(PartialEq, Clone, Debug, Copy)]
struct Vector<T: Float + Copy> {
    x: T, 
    y: T, 
    z: T, 
}

impl<T: Copy + Float> Vector<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/** The sum of two vectors.
 */
impl<T: Copy + Float> ops::Add<Vector<T>> for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/** Subtract a vectro V1 from a vector V2.
 */
impl<T: Copy + Float> ops::Sub<Vector<T>> for Vector<T> {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/** Negate a vector V.
 *
 * Every positive scalar becomes negative and vice versa.
 */
impl<T: Copy + Float> ops::Neg for Vector<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vector::new(-self.x, -self.y, -self.z)
    }
}


/** Multiply a scalar with a Vector.
 */
impl<T: Copy + Float> ops::Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/** Divide a vector by a scalar.
 */
impl<T: Copy + Float> ops::Div<T> for Vector<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}


#[cfg(test)]
mod tests {
    use crate::{Point, Vector};

    #[test]
    fn new_point() {
        assert_eq!(Point{x: 1.0 as f64, y: 2.0 as f64, z: 3.0 as f64 }, Point::<f64>::new(1.0, 2.0, 3.0));
        assert_eq!(Point{x: 1.0 as f32, y: 2.0 as f32, z: 3.0 as f32 }, Point::<f32>::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn new_vector() {
        assert_eq!(Vector{x: 1.0 as f64, y: 2.0 as f64, z: 3.0 as f64 }, Vector::<f64>::new(1.0, 2.0, 3.0));
        assert_eq!(Vector{x: 1.0 as f32, y: 2.0 as f32, z: 3.0 as f32 }, Vector::<f32>::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_point_equality() {
        assert!(Point::new(4.0, -4.0, 3.0) == Point::new(4.0, -4.0, 3.0));
    }

    #[test]
    fn test_vector_equality() {
        assert!(Vector::new(4.3, -4.1, 2.9) == Vector::new(4.3, -4.1, 2.9));
    }

    #[test]
    fn add_vector_to_point() {
        assert_eq!(Point::new(1.0, 1.0, 6.0), Point::new(3.0, -2.0, 5.0) + Vector::new(-2.0, 3.0, 1.0));
    }

    #[test]
    fn add_vector_to_vector() {
        assert_eq!(Vector::new(5.0, 1.0, 6.0), Vector::new(2.0, 3.0, 1.0) + Vector::new(3.0, -2.0, 5.0));
    }

    #[test]
    fn sub_point_from_point() {
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), Point::new(3.0, 2.0, 1.0) - Point::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn sub_vector_from_point() {
        assert_eq!(Point::new(-2.0, -4.0, -6.0), Point::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn sub_vector_from_vector() {
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), Vector::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn negate_vector() {
        assert_eq!(Vector::new(-1.0, 2.0, -3.0), -Vector::new(1.0, -2.0, 3.0));
    }

    #[test]
    fn multiply_vector_by_scalar() {
        assert_eq!(Vector::new(3.5, -7.0, 10.5), Vector::new(1.0, -2.0, 3.0) * 3.5);
    }

    #[test]
    fn multiply_vector_by_scalar_2() {
        assert_eq!(Vector::new(0.5, -1.0, 1.5), Vector::new(1.0, -2.0, 3.0) * 0.5);
    }

    #[test]
    fn divide_vector_by_scalar() {
        assert_eq!(Vector::new(0.5, -1.0, 1.5), Vector::new(1.0, -2.0, 3.0) / 2.0);
    }
}
