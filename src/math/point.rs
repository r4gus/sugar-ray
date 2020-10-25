use super::vector::*;
use std::{ops, cmp};

/// A Point represents a position in 3-dimensional space.
#[derive(Clone, Debug, Copy)]
pub struct Point {
    x: f64, 
    y: f64, 
    z: f64, 
}

impl Point {
    /// Create a new Point.
    ///
    /// # Arguments
    ///
    /// * `x` - Position on the x axis
    /// * `y` - Position on the y axis
    /// * `z` - Position on the z axis
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::math::point::Point;
    ///
    /// let p: Point = Point::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    /// Return the x position of a Point.
    pub fn x(&self) -> f64 {
        self.x 
    }

    /// Return the y position of a Point.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Return the z position of a Point.
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    /// Get the displacement Q of a point P in the direction of and by magnitude of the vector V = Q - P.
    ///
    /// Adds a vector V = (x, y, z) to a point P = (x, y, z) to get it's displacement Q.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The Vecotr to be added
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::math::{point::Point, vector::Vector};
    ///
    /// let p: Point = Point::new(1.0, 2.0, 3.0);
    /// let v: Vector = Vector::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(Point::new(5.0, 7.0, 9.0), p + v);
    /// ```
    fn add(self, rhs: Vector) -> Self {
        Self::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }

}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    /// Get the Vector V between a Point P and a another Point Q.
    ///
    /// Subtracts a point P from a point Q to get a vector V = Q - P.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The Point to be subracted
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::math::{point::Point, vector::Vector};
    ///
    /// let p: Point = Point::new(1.0, 2.0, 3.0);
    /// let q: Point = Point::new(5.0, 7.0, 9.0);
    ///
    /// assert_eq!(Vector::new(4.0, 5.0, 6.0), q - p);
    /// ```
    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Self;
    
    /// Subtract a vector V from a point Q to get it's origin P.
    ///
    /// V = Q - P
    /// P = Q - V
    ///
    /// # Arguments
    ///
    /// * `rhs` - The Vector to be subtracted
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::math::{point::Point, vector::Vector};
    ///
    /// let v: Vector = Vector::new(4.0, 5.0, 6.0);
    /// let q: Point = Point::new(5.0, 7.0, 9.0);
    ///
    /// assert_eq!(Point::new(1.0, 2.0, 3.0), q - v);
    /// ```
    fn sub(self, rhs: Vector) -> Self {
        Self::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= f64::EPSILON &&
        (self.y - other.y).abs() <= f64::EPSILON &&
        (self.z - other.z).abs() <= f64::EPSILON
    }
}





