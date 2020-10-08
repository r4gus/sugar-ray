use std::{
    ops,
    cmp,
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

impl cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.m.len() != other.m.len() {
            // Different row size
            return false;
        }
        
        // iterate over rows
        for r in 0..self.m.len() {
            if self[r].len() != other[r].len() {
                // Different collumn size
                return false;
            }
            
            // iterate over columns
            for c in 0..self[r].len() {
                if (self[r][c] - other[r][c]).abs() > f64::EPSILON {
                    return false;
                }
            }
        }

        true
    }
}












