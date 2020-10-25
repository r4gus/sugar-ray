/// This module defines some transformative operations like __translation__ and __scaling__.
pub mod transformation;

use std::{
    ops,
    cmp,
    marker::Copy,
    clone::Clone,
};

use super::vector::Vector;
use super::point::Point;

/// Represents a __N__ x __M__ Matrix.
///
/// Each element is of type f64.
///
/// # Watch out:
///
/// 1. The multiplication of matrices is associative, i.e. A * (B * C) = (A * B) * C
/// 2. But not commutative, i.e. A * B != B * A
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












