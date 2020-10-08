pub mod point;
pub mod vector;


/* *##############################################################################
 *                               MATRIX ( 4 x 4 )
 * ############################################################################## */

pub struct Matrix {
    m: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}
/* TODO: make a makro to generate matrixes easyly */

#[cfg(test)]
mod tests {
    use crate::math::{
        point::Point, 
        vector::Vector
    };


    #[test]
    fn test_point_equality() {
        assert!(Point::new(4.0, -4.0, 3.0) == Point::new(4.0, -4.0, 3.0));
    }

    #[test]
    fn test_vector_equality() {
        assert!(Vector::new(4.3, -4.1, 2.9) == Vector::new(4.3, -4.1, 2.9));
    }

    #[test]
    fn add_vector_to_point() {
        assert_eq!(Point::new(1.0, 1.0, 6.0), Point::new(3.0, -2.0, 5.0) + Vector::new(-2.0, 3.0, 1.0));
    }

    #[test]
    fn add_vector_to_vector() {
        assert_eq!(Vector::new(5.0, 1.0, 6.0), Vector::new(2.0, 3.0, 1.0) + Vector::new(3.0, -2.0, 5.0));
    }

    #[test]
    fn sub_point_from_point() {
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), Point::new(3.0, 2.0, 1.0) - Point::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn sub_vector_from_point() {
        assert_eq!(Point::new(-2.0, -4.0, -6.0), Point::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn sub_vector_from_vector() {
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), Vector::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0));
    }

    #[test]
    fn negate_vector() {
        assert_eq!(Vector::new(-1.0, 2.0, -3.0), -Vector::new(1.0, -2.0, 3.0));
    }

    #[test]
    fn multiply_vector_by_scalar() {
        assert_eq!(Vector::new(3.5, -7.0, 10.5), Vector::new(1.0, -2.0, 3.0) * 3.5);
    }

    #[test]
    fn multiply_vector_by_scalar_2() {
        assert_eq!(Vector::new(0.5, -1.0, 1.5), Vector::new(1.0, -2.0, 3.0) * 0.5);
    }

    #[test]
    fn divide_vector_by_scalar() {
        assert_eq!(Vector::new(0.5, -1.0, 1.5), Vector::new(1.0, -2.0, 3.0) / 2.0);
    }

    #[test]
    fn magnitude_1() {
        assert_eq!(1.0, Vector::new(1.0, 0.0, 0.0).mag());
    }

    #[test]
    fn magnitude_2() {
        assert_eq!(1.0, Vector::new(0.0, 1.0, 0.0).mag());
    }

    #[test]
    fn magnitude_3() {
        assert_eq!(1.0, Vector::new(0.0, 0.0, 1.0).mag());
    }

    #[test]
    fn magnitude_4() {
        assert_eq!((14.0_f64).sqrt(), Vector::new(1.0, 2.0, 3.0).mag());
    }

    #[test]
    fn magnitude_5() {
        assert_eq!((14.0_f64).sqrt(), Vector::new(-1.0, -2.0, -3.0).mag());
    }

    #[test]
    fn normalizing_vector_1() {
        let mut v = Vector::new(4.0, 0.0, 0.0);
        v.norm();
        assert_eq!(Vector::new(1.0, 0.0, 0.0), v);
    }

    #[test]
    fn normalizing_vector_2() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v.norm();
        assert_eq!(Vector::new(1.0 / (14.0_f64).sqrt(), 2.0 / (14.0_f64).sqrt(), 3.0 / (14.0_f64).sqrt()), v);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        assert_eq!(1.0, Vector::new(1.0, 2.0, 3.0).norm().mag());
    }

    #[test]
    fn dot_product_of_two_vectors() {
        assert_eq!(20.0, Vector::new(1.0, 2.0, 3.0).dot(&Vector::new(2.0, 3.0, 4.0))); 
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(Vector::new(-1.0, 2.0, -1.0), v1.cross(&v2));
        assert_eq!(Vector::new(1.0, -2.0, 1.0), v2.cross(&v1));
    }
}
