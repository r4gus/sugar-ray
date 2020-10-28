pub mod intersection;

use crate::{
    shapes::Sphere,
    math::{
        matrix::Matrix,
        point::Point,
        vector::Vector,
    },
    ray::intersection::{Intersection, Intersections},
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
    /// This function returns always two values `t1` and `t2` as a tuple as an 
    /// Intersections struct if the ray intersects the sphere, where `t1` is the 
    /// smaller value. If the ray intersects the sphere at exactly one point then 
    /// `t1` is equal to `t2`.
    ///
    /// > The function transforms the ray using the assigned `transform` matrix
    /// > of the sphere before doing the calculation (this brings the ray from 
    /// > "world space" into "object space" by applying the inverse transformation, 
    /// > i.e. we don't transform the object itself but the ray).
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
    ///     ray::intersection::{Intersection, Intersections},
    /// };
    ///
    /// let r: Ray = Ray::new(Point::new(0.0,0.0,-5.0), Vector::new(0.0,0.0,1.0));
    /// let s: Sphere = Sphere::new();
    /// let xs: Intersections<'_, Sphere> = r.intersect_sphere(&s).unwrap();
    ///
    /// assert_eq!(4.0, xs[0].t());
    /// assert_eq!(6.0, xs[1].t());
    /// ```
    ///
    /// 2. A ray missing a sphere
    /// ```
    /// # use sugar_ray::{
    /// #    ray::Ray,
    /// #    math::{
    /// #        point::Point, 
    /// #        vector::Vector,
    /// #        },
    /// #    shapes::Sphere,
    /// #    ray::intersection::{Intersection, Intersections},
    /// # };
    ///
    /// let r = Ray::new(Point::new(0.0,-2.0,-5.0), Vector::new(0.0,0.0,1.0));
    /// let s = Sphere::new();
    /// let xs = r.intersect_sphere(&s);
    ///
    /// assert_eq!(true, xs.is_none());
    /// ```
    ///
    /// 3. Intersect sets the object on the intersection
    /// ```
    /// # use sugar_ray::{
    /// #    ray::Ray,
    /// #    math::{
    /// #        point::Point, 
    /// #        vector::Vector,
    /// #        },
    /// #    shapes::Sphere,
    /// #    ray::intersection::{Intersection, Intersections},
    /// # };
    ///
    /// let r = Ray::new(Point::new(0.0,0.0,-5.0), Vector::new(0.0,0.0,1.0));
    /// let s = Sphere::new();
    /// let xs = r.intersect_sphere(&s).unwrap();
    ///
    /// assert_eq!(s, *xs[0].obj());
    /// assert_eq!(s, *xs[1].obj());
    /// ```
    ///
    ///
    /// 4. Intersect a scaled sphere with a ray
    /// ```
    /// use sugar_ray::{
    ///    ray::Ray,
    ///    math::{
    ///        point::Point, 
    ///        vector::Vector,
    ///        matrix::transformation::scaling,
    ///        },
    ///    shapes::Sphere,
    ///    ray::intersection::{Intersection, Intersections},
    /// };
    ///
    /// let r = Ray::new(Point::new(0.0,0.0,-5.0), Vector::new(0.0,0.0,1.0));
    /// let mut s = Sphere::new();
    /// s.set_transform(scaling(2.0, 2.0, 2.0));
    /// let xs = r.intersect_sphere(&s).unwrap();
    ///
    /// assert_eq!(3.0, xs[0].t());
    /// assert_eq!(7.0, xs[1].t());
    /// ```
    ///
    /// 5. Intersecting a translated sphere with a ray
    /// ```
    /// # use sugar_ray::{
    /// #   ray::Ray,
    /// #   math::{
    /// #       point::Point, 
    /// #       vector::Vector,
    /// #       matrix::transformation::translation,
    /// #       },
    /// #   shapes::Sphere,
    /// #   ray::intersection::{Intersection, Intersections},
    /// # };
    ///
    /// let r = Ray::new(Point::new(0.0,0.0,-5.0), Vector::new(0.0,0.0,1.0));
    /// let mut s = Sphere::new();
    /// s.set_transform(translation(5.0, 0.0, 0.0));
    /// let xs = r.intersect_sphere(&s);
    ///
    /// assert_eq!(true, xs.is_none());
    /// ```
    pub fn intersect_sphere<'a>(&self, sphere: &'a Sphere) -> Option<Intersections<'a, Sphere>> {
        let tray = self.transform(&sphere.get_transform().inverse().unwrap());

        // We assume that every sphere has its origin at p(0,0,0).
        let sphere_to_ray = *tray.origin() - Point::new(0.0, 0.0, 0.0);

        let a = tray.direction().dot(&tray.direction());
        let b = 2.0 * tray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b  - 4.0 * a * c;
        
        // If the discriminant is negative, then the ray misses
        // and no intersections occure between the sphere and the ray.
        if discriminant < 0.0 { return None; }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Some(Intersections::new(vec![Intersection::new(t1, sphere),
                                     Intersection::new(t2, sphere)]))
    }
    
    /// Transform a ray.
    ///
    /// Applies the given transformation matrix to the ray.
    ///
    /// # Arguments
    ///
    /// * `m` - The transformation matrix to apply
    ///
    /// The transformation matrix has to be generated with one of
    /// the following funktions: [`translation`], [`scaling`],
    /// [`rotation_rax_x`], [`rotation_rax_y`], [`rotation_rax_z`]
    /// or [`shearing`] from the [`transformation`] module.
    ///
    /// # Examples
    ///
    /// 1. Translating a ray.
    /// ```
    /// use sugar_ray::{
    ///     ray::Ray,
    ///     math::{
    ///         point::Point,
    ///         vector::Vector,
    ///         matrix::transformation::translation,
    ///     },
    /// };
    ///
    /// let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
    /// let m = translation(3.0, 4.0, 5.0);
    /// let r2 = r.transform(&m);
    /// assert_eq!(Point::new(4.0, 6.0, 8.0), *r2.origin());
    /// assert_eq!(Vector::new(0.0, 1.0, 0.0), *r2.direction());
    /// ```
    ///
    /// 2. Scaling a ray
    /// ```
    /// # use sugar_ray::{
    /// #   ray::Ray,
    /// #    math::{
    /// #        point::Point,
    /// #        vector::Vector,
    /// #        matrix::transformation::scaling,
    /// #    },
    /// # };
    ///
    /// let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
    /// let m = scaling(2.0, 3.0, 4.0);
    /// let r2 = r.transform(&m);
    /// assert_eq!(Point::new(2.0, 6.0, 12.0), *r2.origin());
    /// assert_eq!(Vector::new(0.0, 3.0, 0.0), *r2.direction());
    /// ```
    pub fn transform(&self, m: &Matrix) -> Self {
        Self { origin: m.mul_point(&self.origin()), direction: m.mul_vec(&self.direction()) } 
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

        assert_eq!(4.0, xs[0].t());
        assert_eq!(6.0, xs[1].t());
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0,1.0,-5.0), Vector::new(0.0,0.0,1.0));
        let s = Sphere::new();
        let xs = r.intersect_sphere(&s).unwrap();

        assert_eq!(5.0, xs[0].t());
        assert_eq!(5.0, xs[1].t());
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

        assert_eq!(-1.0, xs[0].t());
        assert_eq!(1.0, xs[1].t());
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0.0,0.0,5.0), Vector::new(0.0,0.0,1.0));
        let s = Sphere::new();
        let xs = r.intersect_sphere(&s).unwrap();

        assert_eq!(-6.0, xs[0].t());
        assert_eq!(-4.0, xs[1].t());
    }
}
