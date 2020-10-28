/// Represents a specific intersection between a ray and an object.
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a, T> {
    t: f64,  // A t value wher Origin + t * Direction = Point
    obj: &'a T, // A reference to the intersected object
}

impl<'a, T> Intersection<'a, T> {
    /// Creates a new intersection.
    ///
    /// # Arguments
    ///
    /// * `t` - The `t` value of the intersection, where `Origin +  t * Direction = Point`
    /// * `obj` - Reference to the object that was intersected
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::{
    ///     shapes::Sphere,
    ///     ray::intersection::{Intersection, Intersections},
    /// };
    ///
    ///
    /// let s = Sphere::new();
    /// let i = Intersection::new(3.5, &s);
    ///
    /// assert_eq!(3.5, i.t());
    /// assert_eq!(s, *i.obj());
    /// ```
    pub fn new(t: f64, obj: &'a T) -> Self {
        Self { t, obj }
    }
    
    /// Get the intersections `t` value.
    pub fn t(&self) -> f64 {
        self.t
    }
    
    /// Get a reference to the object that was intersected.
    pub fn obj(&self) -> &'a T {
        self.obj
    }
}

/// Represents a collection of Intersection(s).
#[derive(Debug, PartialEq)]
pub struct Intersections<'a, T> {
    v: Vec<Intersection<'a, T>>,
}

impl<'a, T> Intersections<'a, T> {
    /// Create a new collection of Intersection(s).
    ///
    /// The intersections are sorted by their `t` values
    /// in ascending order.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of intersections
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::{
    ///     shapes::Sphere,
    ///     ray::intersection::{Intersection, Intersections},
    /// };
    ///
    ///
    /// let s = Sphere::new();
    /// let i1 = Intersection::new(2.0, &s);
    /// let i2 = Intersection::new(1.0, &s);
    ///
    /// let xs = Intersections::new(vec![i1, i2]);
    /// assert_eq!(2, xs.len());
    /// assert_eq!(1.0, xs[0].t());
    /// assert_eq!(2.0, xs[1].t());
    /// ```
    pub fn new(mut args: Vec<Intersection<'a, T>>) -> Self {
        args.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());

        Self { v: args }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
    
    /// Returns the hit from a collection of intersection records.
    ///
    /// The hit will always be the intersection with the lowest
    /// nonnegative t value. None is returned, if all intersections
    /// have negative `t` values.
    ///
    /// # Examples
    /// 
    /// 1. `hit` returns the intersection with the smallest positive `t` value.
    /// ```
    /// use sugar_ray::{
    ///     shapes::Sphere,
    ///     ray::intersection::{Intersection, Intersections},
    /// };
    ///
    /// let s = Sphere::new();
    /// let i1 = Intersection::new(1.0, &s);
    /// let i2 = Intersection::new(2.0, &s);
    /// let xs = Intersections::new(vec![i1, i2]);
    ///
    /// assert_eq!(Intersection::new(1.0, &s), *xs.hit().unwrap());
    /// ```
    ///
    /// 2. None is returned if all `t` values are smaller than `0`.
    /// ```
    /// # use sugar_ray::{
    /// #    shapes::Sphere,
    /// #    ray::intersection::{Intersection, Intersections},
    /// # };
    ///
    /// let s = Sphere::new();
    /// let i1 = Intersection::new(-2.0, &s);
    /// let i2 = Intersection::new(-1.0, &s);
    /// let xs = Intersections::new(vec![i1, i2]);
    ///
    /// assert_eq!(true, xs.hit().is_none());
    /// ```
    pub fn hit(&self) -> Option<&Intersection<'a, T>> {
        
        for i in 0..self.len() {
            // hit assumes that the intersections are sorted in ascending order.
            if self[i].t() >= 0.0 {
                return Some(&self[i]);
            }
        }

        None
    }
}

impl<'a, T> std::ops::Index<usize> for Intersections<'a, T> {
    type Output = Intersection<'a, T>;

    fn index(&self, i: usize) -> &Intersection<'a, T> {
        &self.v[i]
    }

}

#[cfg(test)]
mod test {
    use crate::{
        shapes::Sphere,
        ray::intersection::{Intersection, Intersections},
    };

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i1, i2]);
        
        assert_eq!(Intersection::new(1.0, &s), *xs.hit().unwrap());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        assert_eq!(Intersection::new(2.0, &s), *xs.hit().unwrap());
    }
}
