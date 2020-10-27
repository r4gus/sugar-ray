/// Represents a specific intersection between a ray and an object.
#[derive(Debug, PartialEq)]
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
    /// let i1 = Intersection::new(1.0, &s);
    /// let i2 = Intersection::new(2.0, &s);
    ///
    /// let xs = Intersections::new(vec![i1, i2]);
    /// assert_eq!(2, xs.len());
    /// assert_eq!(1.0, xs[0].t());
    /// assert_eq!(2.0, xs[1].t());
    /// ```
    pub fn new(args: Vec<Intersection<'a, T>>) -> Self {
        Self { v: args }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}

impl<'a, T> std::ops::Index<usize> for Intersections<'a, T> {
    type Output = Intersection<'a, T>;

    fn index(&self, i: usize) -> &Intersection<'a, T> {
        &self.v[i]
    }
}
