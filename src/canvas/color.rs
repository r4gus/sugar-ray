use std::{
    ops, 
    cmp,
    f32,
    f64,
};
use num_traits::Float;

/** A RGB color structure.
 *
 * r: red
 * g: green 
 * b: blue
 *
 * The presence (or absence) of each color
 * is represented through a floating point
 * value n, where 0 <= n <= 1.
 *
 * red = (1,0,0)
 * green = (0,1,0)
 * blue = (0,0,1)
 */
#[derive(Debug, Clone, Copy)]
pub struct Color<T: Float + Copy> {
    r: T,
    g: T,
    b: T,
}

impl<T: Copy + Float> Color<T> {
    pub fn new(red: T, green: T, blue: T) -> Self {
        Color { r: red, g: green, b: blue }
    }
}

/** Add two colors.
 */
impl<T: Copy + Float> ops::Add<Color<T>> for Color<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

/** Subtract a color from another.
 */
impl<T: Copy + Float> ops::Sub<Color<T>> for Color<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

/** Multiply a color by a scalar.
 */
impl<T: Copy + Float> ops::Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

/** Blend two colors using the Hadamar Product.
 */
impl<T: Copy + Float> ops::Mul<Color<T>> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl<T: Copy + Float> cmp::PartialEq for Color<T> {
    fn eq(&self, other: &Self) -> bool {
        ((self.r - other.r).abs() < Float::epsilon() ) &&
        ((self.g - other.g).abs() < Float::epsilon() ) &&
        ((self.b - other.b).abs() < Float::epsilon() )
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::color::{Color};

    #[test]
    fn new_color() {
        assert_eq!(Color { r: -0.5, g: 0.4, b: 1.7 }, Color::new(-0.5, 0.4, 1.7));
    }
    
    #[test]
    fn adding_colors() {
        assert_eq!(Color::new(1.6, 0.7, 1.0), Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25));
    }

    #[test]
    fn subtracting_colors() {
        assert_eq!(Color::new(0.2, 0.5, 0.5), Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25));
    }

    #[test]
    fn multiply_color_by_scalar() {
        assert_eq!(Color::new(0.4, 0.6, 0.8), Color::new(0.2, 0.3, 0.4) * 2.0);
    }

    #[test]
    fn multiply_colors() {
        assert_eq!(Color::new(0.9, 0.2, 0.04), Color::new(1.0, 0.2, 0.4) * Color::new(0.9, 1.0, 0.1));
    }
}
