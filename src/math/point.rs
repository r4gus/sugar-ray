use super::vector::*;
use std::ops;

/** (x, y, z) Coordinate representing a point in 3-dimensional space.
 */
#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Point {
    x: f64, 
    y: f64, 
    z: f64, 
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x 
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

/** Get the displacement Q of a point P in the direction of and by magnitude of the vector V = Q - P.
 * 
 * Add a vector V = (x, y, z) to a point P = (x, y, z) to get it's displacement Q.
 *
 */
impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Self::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }

}

/** Subtract a point P from a point Q to get a vector V = Q - P.
 */
impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/** Subtract a vector V from a point Q to get it's origin P.
 *
 * V = Q - P
 * P = Q - V
 */
impl ops::Sub<Vector> for Point {
    type Output = Self;
    
    fn sub(self, rhs: Vector) -> Self {
        Self::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}
