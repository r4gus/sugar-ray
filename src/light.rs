use crate::math::{
    point::Point,
};
use crate::canvas::color::Color;

/// A light source without a size.
///
/// This light source exists at a single point in space
/// and is defined by its `intensity` (how bright it is/ its color)
/// and `position`.
pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    /// Create a new point light with a color and position.
    ///
    /// # Arguments
    ///
    /// * `intensity` - The color / brightness
    /// * `position` - The position of the light source in space
    pub fn new(intensity: Color, position: Point) -> Self {
        Self { intensity, position }
    }
    
    /// Get the point lights intensity.
    pub fn intensity(&self) -> &Color {
        &self.intensity
    }
    
    /// Get the point lights position.
    pub fn position(&self) -> &Point {
        &self.position
    }
}

#[cfg(test)]
mod test {
    use crate::{
        math::{
            vector::Vector,
            point::Point,
        },
        canvas::color::Color,
        light::*,
    };

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let point_light = PointLight::new(intensity, position);
        
        assert_eq!(Color::new(1.0, 1.0, 1.0), *point_light.intensity());
        assert_eq!(Point::new(0.0, 0.0, 0.0), *point_light.position());
    }
}
