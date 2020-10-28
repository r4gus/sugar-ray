use crate::math::matrix::Matrix;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix,
}

impl Sphere {
    /// Create a new Sphere.
    ///
    /// A sphere is always located at the origin with an radius of 1
    /// to ease the calculation of intersections with rays (this can
    /// be seen as kind of a "object space").
    ///
    /// To adjust the location, size and rotations one can use
    /// transformations (those transformations can be seen as 
    /// a conversion from "object space" to "world space". The
    /// inverse of those transformations would do the oposite).
    ///
    /// A sphere has a transformation assigned to it. By default
    /// this transformation is the identity matrix, i.e. there
    /// is no transformation assigned.
    ///
    /// # Examples
    /// 
    /// 1. A sphere's default transformation
    /// ```
    /// use sugar_ray::shapes::Sphere;
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let s: Sphere = Sphere::new();
    /// assert_eq!(Matrix::from_vec(vec![vec![1.0,0.0,0.0,0.0],
    ///                                  vec![0.0,1.0,0.0,0.0],
    ///                                  vec![0.0,0.0,1.0,0.0],
    ///                                  vec![0.0,0.0,0.0,1.0]]).unwrap(), *s.get_transform());
    /// ```
    pub fn new() -> Self {
        Self { transform: Matrix::identity() } 
    }
    
    /// Return the assigned transfromation matrix.
    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }
    
    /// Set a sphere's transformation
    ///
    /// # Arguments
    ///
    /// * `m` - The transformation to set
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::shapes::Sphere;
    /// use sugar_ray::math::matrix::{Matrix, transformation::translation};
    ///
    /// let mut s = Sphere::new();
    /// let t = translation(2.0, 3.0, 4.0);
    /// 
    /// s.set_transform(t);
    /// assert_eq!(translation(2.0, 3.0, 4.0), *s.get_transform());
    /// ```
    pub fn set_transform(&mut self, m: Matrix) {
        self.transform = m;
    }
    
}


