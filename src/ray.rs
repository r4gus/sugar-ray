pub mod intersection;

use crate::{
    shapes::Sphere,
    math::{
        point::Point,
        vector::Vector,
    },
};

/// A ray (or line) with an starting point and a direction.
#[derive(Debug, PartialEq)]
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
    
    /// Get a reference to the origin of the ray.
    pub fn origin(&self) -> &Point {
        &self.origin
    }
    
    /// Get a reference to the direction (vector) of the ray.
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

    /// Calculates the distances at which a specific ray intersects the given sphere.
    ///
    /// This function returns always two values `t1` and `t2` as a tuple `(t1, t2)`
    /// if the ray intersects the sphere, where `t1` is the smaller value. If the
    /// ray intersects the sphere at exactly one point then `t1` is equal to `t2`.
    ///
    /// # Arguments
    ///
    /// * `sphere` - The Sphere to check for intersections with the ray
    ///
    /// # Examples
    ///
    /// 1. A ray intersects a sphere at two points
    /// ```
    /// use sugar_ray::{
    ///     ray::Ray,
    ///     math::{
    ///         point::Point, 
    ///         vector::Vector,
    ///         },
    ///     shapes::Sphere,
    /// };
    ///
    /// let r: Ray = Ray::new(Point::new(0.0,0.0,-5.0), Vector::new(0.0,0.0,1.0));
    /// let s: Sphere = Sphere::new();
    /// let xs: (f64, f64) = r.intersect_sphere(&s).unwrap();
    ///
    /// assert_eq!(4.0, xs.0);
    /// assert_eq!(6.0, xs.1);
    /// ```
    ///
    /// 1. A ray missing a sphere
    /// ```
    /// # use sugar_ray::{
    /// #    ray::Ray,
    /// #    math::{
    /// #        point::Point, 
    /// #        vector::Vector,
    /// #        },
    /// #    shapes::Sphere,
    /// # };
    ///
    /// let r: Ray = Ray::new(Point::new(0.0,-2.0,-5.0), Vector::new(0.0,0.0,1.0));
    /// let s: Sphere = Sphere::new();
    /// let xs: Option<(f64, f64)> = r.intersect_sphere(&s);
    ///
    /// assert_eq!(true, xs.is_none());
    /// ```
    pub fn intersect_sphere(&self, sphere: &Sphere) -> Option<(f64, f64)> {
        // We assume that every sphere has its origin at p(0,0,0).
        let sphere_to_ray = *self.origin() - Point::new(0.0, 0.0, 0.0);

        let a = self.direction().dot(&self.direction());
        let b = 2.0 * self.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b  - 4.0 * a * c;
        
        // If the discriminant is negative, then the ray misses
        // and no intersections occure between the sphere and the ray.
        if discriminant < 0.0 { return None; }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Some((t1, t2))
    }

}

#[cfg(test)]
mod test {
    use crate::{
        shapes::Sphere,
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

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0,0.0,-5.0), Vector::new(0.0,0.0,1.0));
        let s = Sphere::new();
        let xs = r.intersect_sphere(&s).unwrap();

        assert_eq!(4.0, xs.0);
        assert_eq!(6.0, xs.1);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0,1.0,-5.0), Vector::new(0.0,0.0,1.0));
        let s = Sphere::new();
        let xs = r.intersect_sphere(&s).unwrap();

        assert_eq!(5.0, xs.0);
        assert_eq!(5.0, xs.1);
    }

    #[test]
    fn a_ray_missing_a_sphere() {
        let r = Ray::new(Point::new(0.0,2.0,-5.0), Vector::new(0.0,0.0,1.0));
        let s = Sphere::new();
        let xs = r.intersect_sphere(&s);

        assert_eq!(true, xs.is_none());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0,0.0,0.0), Vector::new(0.0,0.0,1.0));
        let s = Sphere::new();
        let xs = r.intersect_sphere(&s).unwrap();

        assert_eq!(-1.0, xs.0);
        assert_eq!(1.0, xs.1);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0.0,0.0,5.0), Vector::new(0.0,0.0,1.0));
        let s = Sphere::new();
        let xs = r.intersect_sphere(&s).unwrap();

        assert_eq!(-6.0, xs.0);
        assert_eq!(-4.0, xs.1);
    }
}
