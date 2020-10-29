use super::point::*;
use std::ops;

/** Vector representing magnitude and direction in 3-dimensional space.
 */
#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Vector {
    x: f64, 
    y: f64, 
    z: f64, 
}

impl Vector {
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
    
    /** Magnitude (length) of a vector from origin P to Q.
     *
     * This function uses the Pythagoras Theorem to calculate the
     * magnitude of a given vector V = (x,y,z).
     */
    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()    
    }

    /** Normalize takes an arbitrary vector and converts it into a unit vector (magnitude = 1).
     *
     * This can help keeping calculations anchored relative to a common scale (the unit vector).
     */
    pub fn norm(&mut self) -> &Self {
        let m = self.mag();
        
        if m != 0.0 {
            self.x = self.x / m;
            self.y = self.y / m;
            self.z = self.z / m;
        }
        self
    }

    /** Normalize takes an arbitrary vector and converts it into a unit vector (magnitude = 1).
     *
     * This can help keeping calculations anchored relative to a common scale (the unit vector).
     */
    pub fn norm_cpy(&self) -> Self {
        let m = self.mag();

        let mut v = Self {  x: self.x, y: self.y, z: self.z };
        
        if m != 0.0 {
            v.x = v.x / m;
            v.y = v.y / m;
            v.z = v.z / m;
        }

        v
    }

    /** The dot product (scalar product) of two vectors.
     *
     * The dot product describes the angle between two vectors.
     * 
     * - Given two unit vectors, a dot product of 1 means the vectors are identical.
     * - -1 means they point in opposite directions
     */
    pub fn dot(&self, vec: &Self) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z 
    }

    /** The cross product of two vectors.
     *
     * Calculates a new vector that is prependicular to both of the original vectors.
     */
    pub fn cross(&self, vec: &Self) -> Self {
        Vector::new(self.y * vec.z - self.z * vec.y,
                    self.z * vec.x - self.x * vec.z,
                    self.x * vec.y - self.y * vec.x)
    }
}

/** The sum of two vectors.
 */
impl ops::Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/** Subtract a vectro V1 from a vector V2.
 */
impl ops::Sub<Vector> for Vector {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/** Negate a vector V.
 *
 * Every positive scalar becomes negative and vice versa.
 */
impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector::new(-self.x, -self.y, -self.z)
    }
}


/** Multiply a scalar with a Vector.
 */
impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/** Divide a vector by a scalar.
 */
impl ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
