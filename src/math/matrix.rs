/// This module defines some transformative operations like __translation__ and __scaling__.
///
/// # Examles
///
/// 1. Individual transformations are applied in sequence.
/// ```
/// use sugar_ray::math::{point::Point, matrix::{Matrix, transformation::*}};
///
/// let mut p = Point::new(1.0, 0.0, 1.0);
/// let r = rotation_rad_x(std::f64::consts::PI / 2.0);
/// let s = scaling(5.0, 5.0, 5.0);
/// let t = translation(10.0, 5.0, 7.0);
///
/// // Rotate by 90 degree around the x axis
/// p = r * p;
///
/// // Then scale it
/// p = s * p;
///
/// // Then move it
/// p = t * p;
///
/// assert_eq!(Point::new(15.0, 0.0, 7.0), p);
/// ```
///
/// 2. Chained transformations must be applied in __reverse order__.
/// ```
/// use sugar_ray::math::{point::Point, matrix::{Matrix, transformation::*}};
///
/// let mut p = Point::new(1.0, 0.0, 1.0);
/// let r = rotation_rad_x(std::f64::consts::PI / 2.0);
/// let s = scaling(5.0, 5.0, 5.0);
/// let t = translation(10.0, 5.0, 7.0);
///
/// // apply multiplication in reverse order
/// let transform = t * s * r;
///
/// assert_eq!(Point::new(15.0, 0.0, 7.0), transform * p);
/// ```
///
/// 3. You can also concatenate transformations in a more natural way through
/// transformation methods defined for Matrix.
/// ```
/// use sugar_ray::math::{point::Point, matrix::{Matrix, transformation::*}};
///
/// let mut p = Point::new(1.0, 0.0, 1.0);
///
/// let transform = Matrix::identity().
///                     rotate_x(std::f64::consts::PI / 2.0).
///                     scale(5.0, 5.0, 5.0).
///                     translate(10.0, 5.0, 7.0);
///
/// assert_eq!(Point::new(15.0, 0.0, 7.0), transform * p);
/// ```
pub mod transformation;

use std::{
    ops,
    cmp,
    marker::Copy,
    clone::Clone,
};

use super::vector::Vector;
use super::point::Point;
use transformation::*;

/// Represents a __N__ x __M__ Matrix.
///
/// Each element is of type f64.
///
/// # Watch out:
///
/// 1. The multiplication of matrices is associative, i.e. A * (B * C) = (A * B) * C
/// 2. But not commutative, i.e. A * B != B * A
///
/// # Access
///
/// A matrix can be accessed in an array like manner using square brackets.
///
/// ```
/// use sugar_ray::math::matrix::Matrix;
///
/// // Create a 2 x 2 matrix
/// let mut m = Matrix::new(2,2);
///
/// m[0][1] = 2.3;
/// m[1][0] = 7.5;
///
/// assert_eq!(Matrix::from_vec(vec![vec![0.0, 2.3], vec![7.5, 0.0]]).unwrap(), m);
/// assert_eq!(7.5, m[1][0]);
/// ```
#[derive(Clone, Debug)]
pub struct Matrix {
    m: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Create a new Matrix
    ///
    /// # Arguments
    ///
    /// * `rows` - The number of rows of the matrix
    /// * `cols` - The number of columns of the matrix
    ///
    /// Each cell is initialized with __0.0__.
    ///
    /// # Examples
    ///
    /// 1. Create a new 4 x 4 matrix
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let m: Matrix = Matrix::new(4, 4);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { m: vec![vec![0 as f64; cols]; rows], rows, cols }
    }
    
    /// Generate a __N__ x __M__ Matrix from an existing vector (Vec).
    ///
    /// # Arguments
    ///
    /// * `v` - The vector to use
    ///
    /// __All rows must have the same length!__
    ///
    /// # Examples
    ///
    /// 1. Create a new 2 x 2 matrix from an existing vector
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let mut m1: Matrix = Matrix::new(2, 2);
    /// m1[0][0] = 1.0;
    /// m1[1][1] = 2.0;
    ///
    /// let mut m2: Matrix = Matrix::from_vec(vec![vec![1.0, 0.0], vec![0.0, 2.0]]).unwrap();
    ///
    /// assert_eq!(m1, m2);
    /// ```
    ///
    /// 2. Try to create a matrix from an __invalid__ vector.
    ///
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// // Rows have a diferent size
    /// let mut m2: Option<Matrix> = Matrix::from_vec(vec![vec![1.0, 0.0], vec![2.0]]);
    ///
    /// assert_eq!(true, m2.is_none());
    /// ```
    pub fn from_vec(v: Vec<Vec<f64>>) -> Option<Matrix> {
        // Non existing Matrix
        if v.len() == 0 {
            None
        } else {
            let row_len = v[0].len();
            
            for row in &v {
                if row.len() != row_len {
                    return None;
                }
            }

            Some(Matrix { rows: v.len(), cols: row_len, m: v })
        }
    }
    
    /// Get the number of rows of the matrix.
    pub fn rows(&self) -> usize {
        self.rows
    }
    
    /// Get the number of columns of the matrix.
    pub fn cols(&self) -> usize {
        self.cols
    }
    
    /// Transposes a given matrix.
    ///
    /// `transpose` switches the rows and columns of a matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let m1 = Matrix::from_vec(vec![vec![0.0,9.0,3.0,0.0], 
    ///                          vec![9.0,8.0,0.0,8.0],
    ///                          vec![1.0,8.0,5.0,3.0],
    ///                           vec![0.0,0.0,5.0,8.0]]).unwrap();
    ///
    /// let expected = Matrix::from_vec(vec![vec![0.0,9.0,1.0,0.0], 
    ///                          vec![9.0,8.0,8.0,0.0],
    ///                          vec![3.0,0.0,5.0,5.0],
    ///                           vec![0.0,8.0,3.0,8.0]]).unwrap();
    ///
    /// assert_eq!(expected, m1.transpose());
    /// ```
    pub fn transpose(&self) -> Self {
        let mut m = Matrix::new(self.rows, self.cols);  

        for r in 0..self.rows {
            for c in 0..self.cols {
                m[r][c] = self[c][r];
            }
        }

        m
    }
    
    /// Find the determinant of a matrix.
    ///
    /// # Examples
    /// 
    /// 1. Find the determinant of a 2 x 2 matrix
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let m = Matrix::from_vec(vec![vec![1.0, 5.0], vec![-3.0,2.0]]).unwrap();
    ///
    ///  assert_eq!(17.0, m.det());
    ///  ```
    pub fn det(&self) -> f64 {
        if self.cols == 2 {
            (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
        } else {
            let mut det = 0.0;

            for c in 0..self.cols {
                det = det + self[0][c] * self.cofactor(0, c);    
            }

            det
        }
    }

    /// Create the submatrix of a given matrix.
    ///
    /// Deletes the n'th row and m'th column of the specified
    /// matrix and returns the remaining rest.
    ///
    /// # Arguments
    ///
    /// * `row` - The row to be deleted
    /// * `col` - The column to be deleted
    ///
    /// # Examples
    ///
    /// 1. Creating a submatrix fo a 3 x 3 matrix
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let m = Matrix::from_vec(vec![vec![1.0,5.0,0.0],
    ///                          vec![-3.0,2.0,7.0],
    ///                          vec![0.0,6.0,-3.0]]).unwrap();
    ///
    /// let expected = Matrix::from_vec(vec![vec![-3.0,2.0],
    ///                          vec![0.0,6.0]]).unwrap();
    ///
    /// // Delete the 0'th row and 2'nd column from m
    /// assert_eq!(expected, m.submatrix(0, 2));
    /// ```
    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut m = Matrix::new(self.rows - 1, self.cols - 1);
        let mut r_new = 0;
        let mut c_new = 0;

        for r in 0..self.rows {
            if r == row {
                // skip over "deleted" row
                continue;
            }
            
            c_new = 0;
            for c in 0..self.cols {
                if c == col {
                    // skip over "deleted" column
                    continue;
                }

                m[r_new][c_new] = self[r][c];
                c_new += 1;
            }

            r_new += 1;
        }

        m
    }
    
    /// Calculate the minor of an element at row `row` and column `col`.
    ///
    /// The minor of an element of the matrix M, M(i,j) is the determinant of the
    /// sub-matrix at (i,j).
    ///
    /// # Arguments
    ///
    /// * `row` - The row
    /// * `col` - The column
    ///
    /// # Examples
    ///
    /// 1. Calculating the minor of a 3 x 3 matrix
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    /// 
    /// let m = Matrix::from_vec(vec![vec![3.0,5.0,0.0],
    ///                          vec![2.0,-1.0,-7.0],
    ///                           vec![6.0,-1.0,5.0]]).unwrap();
    ///
    /// assert_eq!(25.0, m.minor(1, 0));
    /// ```
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }
    
    /// Calculate the cofactor of an element at row `row` and column `col`.
    ///
    /// The cofactor is the (possibly) sign changed minor of the element.
    ///
    /// # Arguments
    ///
    /// * `row` - The row
    /// * `col` - The column
    ///
    /// (row, col) determines the element within the matrix
    /// for which the cofactor is to be calculated.
    ///
    /// # Examples
    /// 
    /// 1. Calculating cofactors of a 3 x 3 matrix
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    ///
    /// let m = Matrix::from_vec(vec![vec![3.0,5.0,0.0],
    ///                          vec![2.0,-1.0,-7.0],
    ///                          vec![6.0,-1.0,5.0]]).unwrap();
    ///
    /// // cofactor for the element m[0][0]
    /// assert_eq!(-12.0, m.cofactor(0, 0));
    /// // cofactor for the element m[1][0]
    /// assert_eq!(-25.0, m.cofactor(1, 0));
    /// ```
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut d = self.minor(row, col);

        if (row +  col) % 2 != 0 {
            d = -d;
        }

        d
    }
    
    /// Checks if the given matrix (is_inv)ersible.
    /// 
    /// A matrix is inversible if it's determinant is
    /// not equal to zero.
    pub fn is_inv(&self) -> bool {
        self.det().abs() != 0.0
    }
    
    /// Calculates the inverse of the given matrix.
    ///
    /// A matrix has an inverse if it's determinant is
    /// not equal to zero.
    /// 
    ///
    /// # Examples
    ///
    /// If you have three matrices __A__, __B__ and __C__ where
    /// __A * B = C__, then you can calculate __A from C__
    /// by __multiplying C by the inverse of B__.
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let a = Matrix::from_vec(vec![vec![3.0,-9.0,7.0,3.0],
    ///                           vec![3.0,-8.0,2.0,-9.0],
    ///                          vec![-4.0,4.0,4.0,1.0],
    ///                          vec![-6.0,5.0,-1.0,1.0]]).unwrap();
    ///
    /// let b = Matrix::from_vec(vec![vec![8.0,2.0,2.0,2.0],
    ///                          vec![3.0,-1.0,7.0,0.0],
    ///                           vec![7.0,0.0,5.0,4.0],
    ///                           vec![6.0,-2.0,0.0,5.0]]).unwrap();
    ///
    /// let c = a.mul(&b);
    /// assert_eq!(a, c.mul(&b.inverse().unwrap()));
    /// ```
    pub fn inverse(&self) -> Option<Self> {
        if !self.is_inv() {
            return None;
        }

        let mut m = Matrix::new(self.rows, self.cols);
        let det = self.det();

        // Setp 1: construct a matrix of cofactors
        // Step 2: Transpose matrix of cofactors (switch rows and columns for each element)
        // Step 3:  Divide each element by the determinant
        //          of the original matrix
        //
        //  All steps are executed in a single loop.
        for r in 0..self.rows {
            for c in 0..self.cols {
                m[c][r] = self.cofactor(r, c) / det;
            }
        }

        Some(m)
    }

    /* Multiplies to matrices.
     *
     * The number of columns of the first matrix have to match
     * the number of rows of the second matrix!
     */
    fn _mul(&self, other: &Self) -> Self {
        assert!(self.cols == other.rows, 
                "Number of Columns of first matrix must be equal to the number of rows of the second.");  
        
        let mut matrix = Matrix::new(self.rows, other.cols);
        let n = self.cols;
        
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                for k in 0..n {
                    matrix[i][j] = matrix[i][j] + (self[i][k] * other[k][j]);
                }
            }
        }

        matrix
    }
    
    /// Multiply two matrices.
    ///
    /// The number of columns of the first matrix mus be equal to the number of
    /// rows of the second matrix.
    ///
    /// This function comes in handy if you need
    /// to preserve the matrices you want to multiply and should
    /// be faster than using the __"*"__ operator.
    ///
    /// # Examples
    ///
    /// 1. Multiply matrix A by matrix B
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let a = Matrix::from_vec(vec![vec![3.0,-9.0,7.0,3.0],
    ///                           vec![3.0,-8.0,2.0,-9.0],
    ///                          vec![-4.0,4.0,4.0,1.0],
    ///                          vec![-6.0,5.0,-1.0,1.0]]).unwrap();
    ///
    /// let b = Matrix::from_vec(vec![vec![8.0,2.0,2.0,2.0],
    ///                          vec![3.0,-1.0,7.0,0.0],
    ///                           vec![7.0,0.0,5.0,4.0],
    ///                           vec![6.0,-2.0,0.0,5.0]]).unwrap();
    ///
    /// let c = a.mul(&b);
    /// ```
    pub fn mul(&self, other: &Self) -> Self {
        self._mul(other)
    }
    
    ///Round each element to its nearest integer.
    pub fn round(&mut self) -> &Self {
        for r in 0..self.rows {
            for c in 0..self.cols {
                self[c][r] = self[c][r].round();
            }
        }

        self
    }
    
    /// Create a 4 x 4 identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use sugar_ray::math::matrix::Matrix;
    ///
    /// let ident = Matrix::identity();
    /// ```
    pub fn identity() -> Self {
        Matrix::from_vec(vec![vec![1.0,0.0,0.0,0.0], 
                              vec![0.0,1.0,0.0,0.0],
                              vec![0.0,0.0,1.0,0.0],
                              vec![0.0,0.0,0.0,1.0]]).unwrap()
    }
    
    /// Apply rotation on the x-axis to the given matrix.
    ///
    /// # Arguments
    ///
    /// * `rad` - The rotation to be applied in radians
    ///
    /// # Examples
    ///
    /// 1. Invoking rotate_x on a identity matrix results in a rotation matrix
    /// ```
    /// use sugar_ray::math::matrix::{Matrix, transformation::rotation_rad_x};
    ///
    /// let t = Matrix::identity().rotate_x(std::f64::consts::PI / 2.0);
    ///
    /// assert_eq!(rotation_rad_x(std::f64::consts::PI / 2.0), t);
    /// ```
    pub fn rotate_x(&self, rad: f64) -> Self {
        let rot = rotation_rad_x(rad);
        rot.mul(self)
    }

    /// Apply rotation on the y-axis to the given matrix.
    ///
    /// # Arguments
    ///
    /// * `rad` - The rotation to be applied in radians
    ///
    /// # Examples
    ///
    /// 1. Invoking rotate_y on a identity matrix results in a rotation matrix
    /// ```
    /// use sugar_ray::math::matrix::{Matrix, transformation::rotation_rad_y};
    ///
    /// let t = Matrix::identity().rotate_y(std::f64::consts::PI / 2.0);
    ///
    /// assert_eq!(rotation_rad_y(std::f64::consts::PI / 2.0), t);
    /// ```
    pub fn rotate_y(&self, rad: f64) -> Self {
        let rot = rotation_rad_y(rad);
        rot.mul(self)
    }

    /// Apply rotation on the z-axis to the given matrix.
    ///
    /// # Arguments
    ///
    /// * `rad` - The rotation to be applied in radians
    ///
    /// # Examples
    ///
    /// 1. Invoking rotate_z on a identity matrix results in a rotation matrix
    /// ```
    /// use sugar_ray::math::matrix::{Matrix, transformation::rotation_rad_z};
    ///
    /// let t = Matrix::identity().rotate_z(std::f64::consts::PI / 2.0);
    ///
    /// assert_eq!(rotation_rad_z(std::f64::consts::PI / 2.0), t);
    /// ```
    pub fn rotate_z(&self, rad: f64) -> Self {
        let rot = rotation_rad_z(rad);
        rot.mul(self)
    }

    /// Apply movement to a matrix
    ///
    /// # Arguments
    ///
    /// * `x` - The movement on the x-axis
    /// * `y` - The movement on the y-axis
    /// * `z` - The movement on the z-axis
    ///
    /// # Examples
    ///
    /// 1. Invoking translate on a identity matrix results in a translation matrix
    /// ```
    /// use sugar_ray::math::matrix::{Matrix, transformation::translation};
    ///
    /// let t = Matrix::identity().translate(5.0, -3.0, 2.0);
    ///
    /// assert_eq!(translation(5.0, -3.0, 2.0), t);
    /// ```
    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        let t = translation(x,y,z);
        t.mul(self)
    }
    
    /// Apply scaling to a matrix.
    ///
    /// # Arguments
    ///
    /// * `x` - The scaling on the x-axis
    /// * `y` - The scaling on the y-axis
    /// * `z` - The scaling on the z-axis
    ///
    /// # Examples
    ///
    /// 1. Invoking scale on a identity matrix results in a scaling matrix
    /// ```
    /// use sugar_ray::math::matrix::{Matrix, transformation::scaling};
    ///
    /// let s = Matrix::identity().scale(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(scaling(2.0, 3.0, 4.0), s);
    /// ```
    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        let s = scaling(x,y,z);
        s.mul(self)
    }

    /// Apply shearing to a matrix.
    ///
    /// # Arguments
    ///
    /// * `xpy` - X value to be moved in proportion to Y
    /// * `xpz` - X value to be moved in proportion to Z
    /// * `ypx` - Y value to be moved in proportion to X
    /// * `ypz` - Y value to be moved in proportion to Z
    /// * `zpx` - Z value to be moved in proportion to X
    /// * `zpy` - Z value to be moved in proportion to Y
    ///
    /// e.g. xpy means "x moved in proportion to y". This
    /// represents the __amount by which to multiply y__ before
    /// __adding it to x__.
    ///
    /// # Examples
    ///
    /// 1. Invoking shear on a identity matrix results in a shearing (or skew) matrix.
    /// ```
    /// use sugar_ray::math::matrix::{Matrix, transformation::shearing};
    ///
    /// let s = Matrix::identity().shear(1.0, 0.0, 2.0, 0.0, 3.0, 0.0);
    ///
    /// assert_eq!(shearing(1.0, 0.0, 2.0, 0.0, 3.0, 0.0), s);
    /// ```
    pub fn shear(&self, xpy: f64, xpz: f64, ypx: f64, ypz: f64, zpx: f64, zpy: f64) -> Self {
        let s = shearing(xpy,xpz,ypx,ypz,zpx,zpy);
        s.mul(self)
    }
}

impl ops::Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, i: usize) -> &Vec<f64> {
        &self.m[i]
    }
}

impl ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.m[i]
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;
    
    /** Multiplies to matrices.
     *
     * The number of columns of the first matrix have to match
     * the number of rows of the second matrix!
     */
    fn mul(self, other: Self) -> Self {
        self._mul(&other)
    }
}

impl ops::Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point::new(
            (self[0][0] * other.x() + self[0][1] * other.y() + self[0][2] * other.z() + self[0][3] * 1.0), 
            (self[1][0] * other.x() + self[1][1] * other.y() + self[1][2] * other.z() + self[1][3] * 1.0),
            (self[2][0] * other.x() + self[2][1] * other.y() + self[2][2] * other.z() + self[2][3] * 1.0))
    }
}

impl ops::Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector::new(




            (self[0][0] * other.x() + self[0][1] * other.y() + self[0][2] * other.z() + self[0][3] * 0.0), 
            (self[1][0] * other.x() + self[1][1] * other.y() + self[1][2] * other.z() + self[1][3] * 0.0),
            (self[2][0] * other.x() + self[2][1] * other.y() + self[2][2] * other.z() + self[2][3] * 0.0))
    }
}

impl cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            // Different row/ col size
            return false;
        }

        //const EPSILON: f64 = f64::EPSILON * 20.0;
        const EPSILON: f64 = 0.00000000000001;
        
        // iterate over rows
        for r in 0..self.rows {
            // iterate over columns
            for c in 0..self.cols {
                if (self[r][c] - other[r][c]).abs() > EPSILON   {
                    return false;
                }
            }
        }

        true
    }
}












