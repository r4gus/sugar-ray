use super::point::*;
use super::vector::*;
use super::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::from_vec(vec![vec![1.0,0.0,0.0,x],
                      vec![0.0,1.0,0.0,y],
                      vec![0.0,0.0,1.0,z],
                      vec![0.0,0.0,0.0,1.0]]).unwrap()
}

#[cfg(test)]
mod test {
    use crate::math::{
        point::Point,
        vector::Vector,
        matrix::Matrix,
        translation::*,
    };

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let t = translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Point::new(2.0,1.0,7.0), t * p);
    }
}
