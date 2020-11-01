use std::{
    ops, 
    cmp,
};
use crate::ppm::PpmColor;

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
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color { r: red, g: green, b: blue }
    }
}

impl PpmColor for Color {
    fn to_ppm_color(&self) -> String {
        const MAX: f32 = 255.0;

        let normalize = |i: f32| -> f32
            {   
                if i < 0.0 {
                    0.0
                } else if i > 1.0 {
                    MAX
                } else {
                    (i * MAX).ceil()
                }
            };

        
        format!("{} {} {}", normalize(self.r), normalize(self.g), normalize(self.b))
    }
}

/** Add two colors.
 */
impl ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

/** Subtract a color from another.
 */
impl ops::Sub<Color> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

/** Multiply a color by a scalar.
 */
impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

/** Blend two colors using the Hadamar Product.
 */
impl ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.r * rhs as f32, self.g * rhs as f32, self.b * rhs as f32)
    }
}

impl cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        ((self.r - other.r).abs() <= f32::EPSILON ) &&
        ((self.g - other.g).abs() <= f32::EPSILON ) &&
        ((self.b - other.b).abs() <= f32::EPSILON )
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::color::{Color};
    use crate::ppm::PpmColor;

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

    #[test]
    fn to_ppm_color_tuple() {
        assert_eq!(String::from("255 0 128"), Color::new(1.0, 0.0, 0.5).to_ppm_color());
    }

    #[test]
    fn to_ppm_color_tuple_2() {
        assert_eq!(String::from("255 204 153"), Color::new(1.0, 0.8, 0.6).to_ppm_color());
    }

    #[test]
    fn to_ppm_color_tuple_3() {
        assert_eq!(String::from("255 128 0"), Color::new(1.5, 0.5, -0.5).to_ppm_color());
    }
}
