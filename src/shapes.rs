use crate::math::{
    matrix::Matrix,
    point::Point,
    vector::Vector,
};

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
    
    /// Calculate the (surface) normal of a sphere at a specific point.
    ///
    /// The surface normal always points perpendicular to a surface at a
    /// given point. 
    ///
    /// # Arguments
    ///
    /// * `world_p` - A point (in world space)
    pub fn normal_at(&self, world_p: Point) -> Vector {
        // First the world point has to be translated into a object point by
        // multiplying it with the inversed transfromation matrix.
        // OP * TMATRIX = WP <=> WP * INV(TMATRIX) = OP
        let object_point  = self.transform.inverse().unwrap().mul_point(&world_p);

        // Then we calculate the (surface normal) which is just the vector from the
        // origin in object space (0, 0, 0) to the calculated object point.
        let object_normal = object_point - Point::new(0.0, 0.0, 0.0);

        // Now this vector has to be translated from object space back to world space.
        // We can't just multiply the vector by the transformation matrix or the normal
        // won't be preserved! Instead we have to multiply it by the transposed, inversed
        // transformation matrix.
        let mut world_normal = self.transform.inverse().unwrap().transpose().mul_vec(&object_normal);

        world_normal.norm(); // normalize the resulting vector
        world_normal
    }
}

#[cfg(test)]
mod test {
    use crate::{
        shapes::Sphere,
        math::{
            point::Point, 
            vector::Vector,
            matrix::{
                Matrix,
                transformation::{translation, scaling, rotation_rad_z},
            },
        },
    };

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(Vector::new(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(Vector::new(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(Vector::new(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0));
        assert_eq!(Vector::new(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0), n);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let mut n = s.normal_at(Point::new(3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0, 3.0_f64.sqrt()/3.0));
        assert_eq!(n, n.norm_cpy());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(Vector::new(0.0, 0.7071067811865475, -0.7071067811865476), n);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = scaling(1.0, 0.5, 1.0) * rotation_rad_z(std::f64::consts::PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(Point::new(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0));
        assert_eq!(Vector::new(0.0, 0.9701425001453319, -0.24253562503633294), n);
    }
}


