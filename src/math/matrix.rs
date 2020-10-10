use std::{
    ops,
    cmp,
    marker::Copy,
    clone::Clone,
};

/** Represents a NxM Matrix.
 */
#[derive(Clone, Debug)]
pub struct Matrix {
    m: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /** Make a NxM Matrix.
     */
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { m: vec![vec![0 as f64; cols]; rows], rows, cols }
    }
    
    /** Generate a NxM Matrix from a nested Vector.
     *
     * All nested elements must have the same length!
     */
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

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
    
    /** Transposes a given matrix.
     *
     * Rows and columns are switched.
     */
    pub fn transpose(&self) -> Self {
        let mut m = Matrix::new(self.rows, self.cols);  

        for r in 0..self.rows {
            for c in 0..self.cols {
                m[r][c] = self[c][r];
            }
        }

        m
    }
    
    /** Find the determinant of a matrix.
     */
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

    /** Create a submatrix of a given matrix.
     *
     * Deletes the n'th row and m'th column of the specified
     * matrix and returns the remaining rest.
     */
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
    
    /** Calculate the minor of an element at row `row` and column `col`.
     *
     * The minor of an element of the matrix M, M(i,j) is the determinant of the
     * sub-matrix at (i,j).
     */
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }
    
    /** Calculate the cofactor of an element at row `row` and column `col`.
     *
     * The cofactor is the (possibly) sign changed minor of the element.
     */
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut d = self.submatrix(row, col).det();

        if (row +  col) % 2 != 0 {
            d = -d;
        }

        d
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
}

impl cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            // Different row/ col size
            return false;
        }
        
        // iterate over rows
        for r in 0..self.rows {
            // iterate over columns
            for c in 0..self.cols {
                if (self[r][c] - other[r][c]).abs() > f64::EPSILON {
                    return false;
                }
            }
        }

        true
    }
}












