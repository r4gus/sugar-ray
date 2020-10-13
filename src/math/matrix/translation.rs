use crate::math::{
    point::Point,
    vector::Vector,
    matrix::Matrix,
};

/** Create a 4x4 translation matrix.
 *
 * A translation matrix multiplied by a Point
 * moves that point according to the x, y and z
 * coordinates of the translation matrix.
 *
 * It has no effect on vectors!
 */
pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::from_vec(vec![vec![1.0,0.0,0.0,x],
                      vec![0.0,1.0,0.0,y],
                      vec![0.0,0.0,1.0,z],
                      vec![0.0,0.0,0.0,1.0]]).unwrap()
}

/** Scale a Point/ Vector by the given values.
 *
 * Scaling by a negative value is essentially a reflection.
 */
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::from_vec(vec![vec![x,0.0,0.0,0.0],
                      vec![0.0,y,0.0,0.0],
                      vec![0.0,0.0,z,0.0],
                      vec![0.0,0.0,0.0,1.0]]).unwrap()
}

#[cfg(test)]
mod test {
    use crate::math::{
        point::Point,
        vector::Vector,
        matrix::{
            Matrix,
            translation::*,
        },
    };

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let t = translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Point::new(2.0,1.0,7.0), t * p);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let t = translation(5.0,-3.0,2.0).inverse().unwrap();
        let p = Point::new(-3.0,4.0,5.0);
        assert_eq!(Point::new(-8.0,7.0,3.0), t * p);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = translation(5.0,-3.0,2.0);
        let v = Vector::new(-3.0,4.0,5.0);
        assert_eq!(Vector::new(-3.0,4.0,5.0), t * v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let t = scaling(2.0,3.0,4.0);
        let p = Point::new(-4.0,6.0,8.0);
        assert_eq!(Point::new(-8.0,18.0,32.0), t * p);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let t = scaling(2.0,3.0,4.0);
        let p = Vector::new(-4.0,6.0,8.0);
        assert_eq!(Vector::new(-8.0,18.0,32.0), t * p);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let t = scaling(2.0,3.0,4.0).inverse().unwrap();
        let v = Vector::new(-4.0,6.0,8.0);
        assert_eq!(Vector::new(-2.0,2.0,2.0), t * v);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let t = scaling(-1.0,1.0,1.0).inverse().unwrap();
        let v = Point::new(2.0,3.0,4.0);
        assert_eq!(Point::new(-2.0,3.0,4.0), t * v);
    }
}
