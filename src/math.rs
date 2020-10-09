pub mod point;
pub mod vector;
pub mod matrix;


#[cfg(test)]
mod tests {
    use crate::math::{
        point::Point, 
        vector::Vector,
        matrix::Matrix,
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

    #[test]
    fn constructing_a_4x4_matrix() {
        let m = Matrix::new(4, 4);
        assert_eq!(4, m.rows());
        assert_eq!(4, m.cols());
    }

    #[test]
    fn constructing_a_4x4_matrix_from_vector() {
        let m = Matrix::from_vec(vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.5,6.5,7.5,8.5],
                                 vec![9.0,10.0,11.0,12.0],
                                 vec![13.5, 14.5, 15.5, 16.5]]).unwrap();
        assert_eq!(1.0, m[0][0]);
        assert_eq!(4.0, m[0][3]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(7.5, m[1][2]);
        assert_eq!(11.0, m[2][2]);
        assert_eq!(13.5, m[3][0]);
        assert_eq!(15.5, m[3][2]);
    }

    #[test]
    fn try_constructing_a_empty_matrix() {
        let m = Matrix::from_vec(Vec::<Vec<f64>>::new());
        assert!(m.is_none());
    }

    #[test]
    fn try_constructing_a_matrix_with_different_row_length() {
        let m = Matrix::from_vec(vec![vec![1.0,2.0,3.0,4.0],
                                 vec![5.5,6.5,7.5]]);
        assert!(m.is_none());
    }
    
    #[test]
    fn assigning_values_to_a_2x2_matrix() {
        let mut m = Matrix::new(2, 2);
        m[0][0] = 1.0;
        m[0][1] = 2.0;
        m[1][0] = 5.5;
        m[1][1] = 6.5;

        assert_eq!(1.0, m[0][0]);
        assert_eq!(2.0, m[0][1]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(6.5, m[1][1]);
    }

    #[test]
    fn constructing_a_2x2_matrix() {
        let m = Matrix::from_vec(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]).unwrap();

        assert_eq!(-3.0, m[0][0]);
        assert_eq!(5.0, m[0][1]);
        assert_eq!(1.0, m[1][0]);
        assert_eq!(-2.0, m[1][1]);
    }

    #[test]
    fn constructing_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![vec![-3.0, 5.0, 0.0], 
                                 vec![1.0, -2.0, -7.0],
                                 vec![0.0, 1.0, 1.0]]).unwrap();

        assert_eq!(-3.0, m[0][0]);
        assert_eq!(-2.0, m[1][1]);
        assert_eq!(1.0, m[2][2]);
    }

    #[test]
    fn comparing_two_equal_matrices() {
        let m1 = Matrix::from_vec(vec![vec![-3.0, 1.6, 0.0], 
                                 vec![1.0, -2.0, -7.0],
                                 vec![0.0, 1.5, 1.0]]).unwrap();

        let m2 = Matrix::from_vec(vec![vec![-3.0, 1.6, 0.0], 
                                 vec![1.0, -2.0, -7.0],
                                 vec![0.0, 1.5, 1.0]]).unwrap();

        assert_eq!(true, m1 == m2);
    }

    #[test]
    fn comparing_two_unequal_matrices() {
        let m1 = Matrix::from_vec(vec![vec![-3.0, 1.6, 0.0], 
                                 vec![1.0, -2.0, -7.0],
                                 vec![0.0, 1.5, 1.0]]).unwrap();

        let m2 = Matrix::from_vec(vec![vec![-2.99999, 1.6, 0.0], 
                                 vec![1.0, -3.0, -7.0],
                                 vec![0.0, 1.5, 1.00001]]).unwrap();

        assert_eq!(false, m1 == m2);
    }

    #[test]
    fn comparing_two_unequal_matrices_2() {
        let m1 = Matrix::from_vec(vec![vec![-3.0, 1.6, 0.0], 
                                 vec![1.0, -2.0, -7.0],
                                 vec![0.0, 1.5, 1.0]]).unwrap();

        let m2 = Matrix::from_vec(vec![vec![-3.0, 0.0], 
                                 vec![1.0, -7.0],
                                 vec![0.0, 1.0]]).unwrap();

        assert_eq!(false, m1 == m2);
    }

    #[test]
    fn comparing_two_unequal_matrices_3() {
        let m1 = Matrix::from_vec(vec![vec![-3.0, 1.6, 0.0], 
                                 vec![1.0, -2.0, -7.0],
                                 vec![0.0, 1.5, 1.0]]).unwrap();

        let m2 = Matrix::from_vec(vec![vec![-3.0, 1.6, 0.0], 
                                 vec![1.0, -2.0, -7.0],
                                 vec![0.0, 1.5, 1.0],
                                 vec![1.0,2.0,3.0]]).unwrap();

        assert_eq!(true, m1 != m2);
    }


    #[test]
    fn multiply_two_matrices() {
        let m1 = Matrix::from_vec(vec![vec![1.0,2.0,3.0,4.0], 
                                 vec![5.0,6.0,7.0,8.0],
                                 vec![9.0,8.0,7.0,6.0],
                                 vec![5.0,4.0,3.0,2.0]]).unwrap();

        let m2 = Matrix::from_vec(vec![vec![-2.0,1.0,2.0,3.0], 
                                 vec![3.0,2.0,1.0,-1.0],
                                 vec![4.0,3.0,6.0,5.0],
                                 vec![1.0,2.0,7.0,8.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![20.0,22.0,50.0,48.0], 
                                 vec![44.0,54.0,114.0,108.0],
                                 vec![40.0,58.0,110.0,102.0],
                                 vec![16.0,26.0,46.0,42.0]]).unwrap();

        assert_eq!(expected, m1 * m2);
    }

}
