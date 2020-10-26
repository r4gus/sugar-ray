use crate::math::{
    point::Point,
    vector::Vector,
};

/// A ray (or line) with an starting point and a direction.
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    /// Create a new Ray.
    ///
    /// # Arguments 
    ///
    /// * `origin` - The starting point
    /// * `direction` - The direction of the ray
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::ray::Ray;
    /// use sugar_ray::math::{point::Point, vector::Vector};
    ///
    /// let r: Ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(4.0, 5.0, 6.0));
    /// ```
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }
    
    /// Get a reference to the origin of a ray.
    pub fn origin(&self) -> &Point {
        &self.origin
    }
    
    /// Get a reference to the vector of a ray.
    pub fn direction(&self) -> &Vector {
        &self.direction
    }
    
    /// Computes the point at the given distance `t` along the ray.
    ///
    /// # Arguments
    ///
    /// * `t` - The distance from the origin
    ///
    /// `t` can be thought of as a time unit. Each time
    /// unit the ray advances further by the distance
    /// described by its vector (direction).
    ///
    /// # Examples
    /// ```
    /// use sugar_ray::ray::Ray;
    /// use sugar_ray::math::{point::Point, vector::Vector};
    ///
    /// let r: Ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(4.0, 5.0, 6.0));
    ///
    /// assert_eq!(Point::new(7.0, 9.5, 12.0), r.position(1.5));
    /// ```
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ray::Ray,
        math::{
        point::Point,
        vector::Vector,
    }};

    #[test]
    fn creating_and_querying_a_ray() {
        let r = Ray::new(Point::new(1.0,2.0,3.0), Vector::new(4.0,5.0,6.0));
        assert_eq!(Point::new(1.0,2.0,3.0), *r.origin());
        assert_eq!(Vector::new(4.0,5.0,6.0), *r.direction());
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2.0,3.0,4.0), Vector::new(1.0,0.0,0.0));

        assert_eq!(Point::new(2.0,3.0,4.0), r.position(0.0));
        assert_eq!(Point::new(3.0,3.0,4.0), r.position(1.0));
        assert_eq!(Point::new(1.0,3.0,4.0), r.position(-1.0));
        assert_eq!(Point::new(4.5,3.0,4.0), r.position(2.5));
    }
}
