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


    #[test]
    fn a_matrix_multiplied_by_vec() {
        let m1 = Matrix::from_vec(vec![vec![1.0,2.0,3.0,4.0], 
                                 vec![2.0,4.0,4.0,2.0],
                                 vec![8.0,6.0,4.0,1.0],
                                 vec![0.0,0.0,0.0,1.0]]).unwrap();

        let m2 = Matrix::from_vec(vec![vec![1.0], 
                                 vec![2.0],
                                 vec![3.0],
                                 vec![1.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![18.0], 
                                 vec![24.0],
                                 vec![33.0],
                                 vec![1.0]]).unwrap();

        assert_eq!(expected, m1 * m2);
    }

    #[test]
    fn multiplying_a_identity_matrix() {
        let m1 = Matrix::from_vec(vec![vec![0.0,1.0,2.0,4.0], 
                                 vec![1.0,2.0,4.0,8.0],
                                 vec![2.0,4.0,8.0,16.0],
                                 vec![4.0,8.0,16.0,32.0]]).unwrap();

        let m2 = Matrix::from_vec(vec![vec![1.0,0.0,0.0,0.0], 
                                 vec![0.0,1.0,0.0,0.0],
                                 vec![0.0,0.0,1.0,0.0],
                                 vec![0.0,0.0,0.0,1.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![0.0,1.0,2.0,4.0], 
                                 vec![1.0,2.0,4.0,8.0],
                                 vec![2.0,4.0,8.0,16.0],
                                 vec![4.0,8.0,16.0,32.0]]).unwrap();

        assert_eq!(expected, m1 * m2);
    }

    #[test]
    fn transposing_a_matrix() {
        let m1 = Matrix::from_vec(vec![vec![0.0,9.0,3.0,0.0], 
                                 vec![9.0,8.0,0.0,8.0],
                                 vec![1.0,8.0,5.0,3.0],
                                 vec![0.0,0.0,5.0,8.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![0.0,9.0,1.0,0.0], 
                                 vec![9.0,8.0,8.0,0.0],
                                 vec![3.0,0.0,5.0,5.0],
                                 vec![0.0,8.0,3.0,8.0]]).unwrap();

        assert_eq!(expected, m1.transpose());
    }

    #[test]
    fn transposing_identity_matrix() {
        let m1 = Matrix::from_vec(vec![vec![1.0,0.0,0.0,0.0], 
                                 vec![0.0,1.0,0.0,0.0],
                                 vec![0.0,0.0,1.0,0.0],
                                 vec![0.0,0.0,0.0,1.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![1.0,0.0,0.0,0.0], 
                                 vec![0.0,1.0,0.0,0.0],
                                 vec![0.0,0.0,1.0,0.0],
                                 vec![0.0,0.0,0.0,1.0]]).unwrap();

        assert_eq!(expected, m1.transpose());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let m = Matrix::from_vec(vec![vec![1.0, 5.0], vec![-3.0,2.0]]).unwrap();

        assert_eq!(17.0, m.det());
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![vec![1.0,5.0,0.0],
                                 vec![-3.0,2.0,7.0],
                                 vec![0.0,6.0,-3.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![-3.0,2.0],
                                 vec![0.0,6.0]]).unwrap();

        assert_eq!(expected, m.submatrix(0, 2));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix() {
        let m = Matrix::from_vec(vec![vec![-6.0,1.0,1.0,6.0],
                                 vec![-8.0,5.0,8.0,6.0],
                                 vec![-1.0,0.0,8.0,2.0],
                                 vec![-7.0,1.0,-1.0,1.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![-6.0,1.0,6.0],
                                 vec![-8.0,8.0,6.0],
                                 vec![-7.0,-1.0,1.0]]).unwrap();

        assert_eq!(expected, m.submatrix(2, 1));
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![vec![3.0,5.0,0.0],
                                 vec![2.0,-1.0,-7.0],
                                 vec![6.0,-1.0,5.0]]).unwrap();

        assert_eq!(25.0, m.minor(1, 0));
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![vec![3.0,5.0,0.0],
                                 vec![2.0,-1.0,-7.0],
                                 vec![6.0,-1.0,5.0]]).unwrap();

        assert_eq!(-12.0, m.cofactor(0, 0));
        assert_eq!(-25.0, m.cofactor(1, 0));
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![vec![1.0,2.0,6.0],
                                 vec![-5.0,8.0,-4.0],
                                 vec![2.0,6.0,4.0]]).unwrap();

        assert_eq!(56.0, m.cofactor(0, 0));
        assert_eq!(12.0, m.cofactor(0, 1));
        assert_eq!(-46.0, m.cofactor(0, 2));
        assert_eq!(-196.0, m.det());
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let m = Matrix::from_vec(vec![vec![-2.0,-8.0,3.0,5.0],
                                 vec![-3.0,1.0,7.0,3.0],
                                 vec![1.0,2.0,-9.0,6.0],
                                 vec![-6.0,7.0,7.0,-9.0]]).unwrap();

        assert_eq!(690.0, m.cofactor(0, 0));
        assert_eq!(447.0, m.cofactor(0, 1));
        assert_eq!(210.0, m.cofactor(0, 2));
        assert_eq!(51.0, m.cofactor(0, 3));
        assert_eq!(-4071.0, m.det());
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let m = Matrix::from_vec(vec![vec![6.0,4.0,4.0,4.0],
                                 vec![5.0,5.0,7.0,6.0],
                                 vec![4.0,-9.0,3.0,-7.0],
                                 vec![9.0,1.0,7.0,-6.0]]).unwrap();

        assert_eq!(true, m.is_inv());
    }

    #[test]
    fn testing_an_noninvertible_matrix_for_invertibility() {
        let m = Matrix::from_vec(vec![vec![-4.0,2.0,-2.0,-3.0],
                                 vec![9.0,6.0,2.0,6.0],
                                 vec![0.0,-5.0,1.0,-5.0],
                                 vec![0.0,0.0,0.0,0.0]]).unwrap();

        assert_eq!(false, m.is_inv());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let m = Matrix::from_vec(vec![vec![-5.0,2.0,6.0,-8.0],
                                 vec![1.0,-5.0,1.0,8.0],
                                 vec![7.0,7.0,-6.0,-7.0],
                                 vec![1.0,-3.0,7.0,4.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![0.21804511278195488, 0.45112781954887216, 0.24060150375939848, -0.045112781954887216], 
                                        vec![-0.8082706766917294, -1.4567669172932332, -0.44360902255639095, 0.5206766917293233], 
                                        vec![-0.07894736842105263, -0.2236842105263158, -0.05263157894736842, 0.19736842105263158], 
                                        vec![-0.5225563909774437, -0.8139097744360902, -0.3007518796992481, 0.30639097744360905]]).unwrap();

        let b = m.inverse().unwrap();

        assert_eq!(532.0, m.det());
        assert_eq!(-160.0, m.cofactor(2,3));
        assert_eq!(105.0, m.cofactor(3,2));
        assert_eq!(-160.0 / 532.0, b[3][2]);
        assert_eq!(105.0 / 532.0, b[2][3]);
        assert_eq!(expected, b);
    }

    #[test]
    fn calculate_inverse_of_another_matrix() {
        let m = Matrix::from_vec(vec![vec![8.0,-5.0,9.0,2.0],
                                 vec![7.0,5.0,6.0,1.0],
                                 vec![-6.0,0.0,9.0,6.0],
                                 vec![-3.0,0.0,-9.0,-4.0]]).unwrap();

        let expected = Matrix::from_vec(vec![vec![-0.15384615384615385, -0.15384615384615385, -0.28205128205128205, -0.5384615384615384], vec![-0.07692307692307693, 0.12307692307692308, 0.02564102564102564, 0.03076923076923077], vec![0.358974358974359, 0.358974358974359, 0.4358974358974359, 0.9230769230769231], vec![-0.6923076923076923, -0.6923076923076923, -0.7692307692307693, -1.9230769230769231]]).unwrap();


        let b = m.inverse().unwrap();

        assert_eq!(expected, b);
    }

    #[test]
    fn multiply_a_product_by_its_inverse() {
        let a = Matrix::from_vec(vec![vec![3.0,-9.0,7.0,3.0],
                                 vec![3.0,-8.0,2.0,-9.0],
                                 vec![-4.0,4.0,4.0,1.0],
                                 vec![-6.0,5.0,-1.0,1.0]]).unwrap();

        let b = Matrix::from_vec(vec![vec![8.0,2.0,2.0,2.0],
                                 vec![3.0,-1.0,7.0,0.0],
                                 vec![7.0,0.0,5.0,4.0],
                                 vec![6.0,-2.0,0.0,5.0]]).unwrap();

        let c = a.mul(&b);
        println!("=====================> {}", f64::EPSILON * 16.0);
        assert_eq!(a, c.mul(&b.inverse().unwrap()));
    }
}
