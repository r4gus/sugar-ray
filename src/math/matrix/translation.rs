use crate::math::{
    point::Point,
    vector::Vector,
    matrix::Matrix,
};

/// Create a 4 x 4 translation matrix.
///
/// A translation matrix multiplied by a Point
/// moves that point according to the x, y and z
/// coordinates of the translation matrix.
///
/// It has no effect on vectors!
///
/// # Arguments
///
/// * `x` - Translation to be applied to the x coordinate
/// * `y` - Translation to be applied to the y coordinate
/// * `z` - Translation to be applied to the z coordinate
///
/// # Examples
///
/// ```
/// use sugar_ray::math::{point::Point, matrix::{Matrix, translation::*}};
///
/// let t = translation(5.0, -3.0, 2.0);
/// let p = Point::new(-3.0, 4.0, 5.0);
///
/// assert_eq!(Point::new(2.0,1.0,7.0), t * p);
/// ```
pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::from_vec(vec![vec![1.0,0.0,0.0,x],
                      vec![0.0,1.0,0.0,y],
                      vec![0.0,0.0,1.0,z],
                      vec![0.0,0.0,0.0,1.0]]).unwrap()
}

/// Create a 4 x 4 scaling matrix.
///
/// A point multiplied by a scaling matrix is moved outwards
/// if the scale value is greater than 1 or inwards if the
/// scaling value is less than 1.
///
/// Scaling by a negative value is essentially a reflection.
///
/// You can scale both points and vectors.
///
/// # Arguments
///
/// * `x` - Scaling to be applied to the x coordinate
/// * `y` - Scaling to be applied to the y coordinate
/// * `z` - Scaling to be applied to the z coordinate
///
/// # Examples
///
/// ```
/// use sugar_ray::math::{point::Point, vector::Vector, matrix::{Matrix, translation::*}};
///
/// let t = scaling(2.0,3.0,4.0);
/// let p = Point::new(-4.0,6.0,8.0);
/// let v = Vector::new(-4.0,6.0,8.0);
///
/// // Scaling a point
/// assert_eq!(Point::new(-8.0,18.0,32.0), t.clone() * p);
/// 
/// // Scaling a vector
/// assert_eq!(Vector::new(-8.0,18.0,32.0), t * v);
/// ```
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::from_vec(vec![vec![x,0.0,0.0,0.0],
                      vec![0.0,y,0.0,0.0],
                      vec![0.0,0.0,z,0.0],
                      vec![0.0,0.0,0.0,1.0]]).unwrap()
}

/// Translate degree into radians.
///
/// 360 deg = 2 * PI
///
/// # Arguments
///
/// * `deg` - An angle in degrees
///
/// # Examples
///
/// ```
/// use sugar_ray::math::matrix::translation::radians;
///
/// assert_eq!(std::f64::consts::PI, radians(180.0))
/// ```
pub fn radians(deg: f64) -> f64 {
    (deg / 180.0) * std::f64::consts::PI
}

/// Generate a rotation Matrix for the x axis.
/// 
/// A Point multiplied by this matrix gets rotated
/// around the x axis (applying the left hand rule).
///
/// # Arguments
///
/// * `r` - The rotation to be applied in __radians__
///
/// # Examples
///
/// ```
/// use sugar_ray::math::{point::Point, matrix::{Matrix, translation::*}};
///
/// let p = Point::new(0.0, 1.0, 0.0);
/// let half_quarter = rotation_rad_x(std::f64::consts::PI / 4.0);
///
/// // P(0, 1, 0) -> P(0, sqrt(2)/2, sqrt(2)/2)
/// assert_eq!(Point::new(0.0, (2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0), half_quarter * p);
/// ```
pub fn rotation_rad_x(r: f64) -> Matrix {
    Matrix::from_vec(vec![vec![1.0,0.0,0.0,0.0],
                      vec![0.0,r.cos(),-r.sin(),0.0],
                      vec![0.0,r.sin(),r.cos(),0.0],
                      vec![0.0,0.0,0.0,1.0]]).unwrap()
}

/// Generate a rotation Matrix for the y axis.
///
/// A Point multiplied by this matrix gets rotated
/// around the y axis (applying the left hand rule).
///
/// # Arguments
///
/// * `r` - The rotation to be applied in __radians__
///
/// # Examples
///
/// ```
/// use sugar_ray::math::{point::Point, matrix::{Matrix, translation::*}};
///
/// let p = Point::new(0.0, 0.0, 1.0);
/// let half_quarter = rotation_rad_y(std::f64::consts::PI / 4.0);
///
/// // P(0, 0, 1) -> P(sqrt(2)/2, 0, sqrt(2)/2)
/// assert_eq!(Point::new((2.0 as f64).sqrt() / 2.0, 0.0, (2.0 as f64).sqrt() / 2.0), half_quarter * p);
/// ```
pub fn rotation_rad_y(r: f64) -> Matrix {
    Matrix::from_vec(vec![vec![r.cos(),0.0,r.sin(),0.0],
                      vec![0.0,1.0,0.0,0.0],
                      vec![-r.sin(),0.0,r.cos(),0.0],
                      vec![0.0,0.0,0.0,1.0]]).unwrap()
}

/// Generate a rotation Matrix for the z axis.
///
/// A Point multiplied by this matrix gets rotated
/// around the z axis (applying the left hand rule).
///
/// # Arguments
///
/// * `r` - The rotation to be applied in __radians__
///
/// # Examples
///
/// ```
/// use sugar_ray::math::{point::Point, matrix::{Matrix, translation::*}};
///
/// let p = Point::new(0.0, 1.0, 0.0);
/// let half_quarter = rotation_rad_z(std::f64::consts::PI / 4.0);
///
/// // P(0, 1, 0) -> P(-sqrt(2)/2, sqrt(2)/2, 0)
/// assert_eq!(Point::new(-(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0, 0.0), half_quarter * p);
/// ```
pub fn rotation_rad_z(r: f64) -> Matrix {
    Matrix::from_vec(vec![vec![r.cos(),-r.sin(),0.0,0.0],
                      vec![r.sin(),r.cos(),0.0,0.0],
                      vec![0.0,0.0,1.0,0.0],
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

    #[test]
    fn degree_to_radians() {
        assert_eq!(std::f64::consts::PI, radians(180.0));
        assert_eq!(std::f64::consts::PI * 2.0, radians(360.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Point::new(0.0,1.0,0.0);
        let half_quarter = rotation_rad_x(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_rad_x(std::f64::consts::PI / 2.0);

        assert_eq!(Point::new(0.0, (2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0), half_quarter * p);
        assert_eq!(Point::new(0.0, 0.0, 1.0), full_quarter * p);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_oposite_direction() {
        let p = Point::new(0.0,1.0,0.0);
        let mut half_quarter = rotation_rad_x(std::f64::consts::PI / 4.0);
        half_quarter = half_quarter.inverse().unwrap();

        assert_eq!(Point::new(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0), half_quarter * p);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Point::new(0.0,0.0,1.0);
        let half_quarter = rotation_rad_y(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_rad_y(std::f64::consts::PI / 2.0);

        assert_eq!(Point::new((2.0 as f64).sqrt() / 2.0, 0.0, (2.0 as f64).sqrt() / 2.0), half_quarter * p);
        assert_eq!(Point::new(1.0, 0.0, 0.0), full_quarter * p);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Point::new(0.0,1.0,0.0);
        let half_quarter = rotation_rad_z(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_rad_z(std::f64::consts::PI / 2.0);

        assert_eq!(Point::new(-(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0, 0.0), half_quarter * p);
        assert_eq!(Point::new(-1.0, 0.0, 0.0), full_quarter * p);
    }
}
