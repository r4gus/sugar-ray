use crate::canvas::color::*;
use crate::light::*;
use crate::math::{
    vector::Vector,
    point::Point,
};

/// A material encapsulates specific surface properties
/// like `color`, `ambient`, `diffuse`, `specular` and
/// `shininess`.
///
/// # Properties
///
/// * `color` - The surface color
/// * `ambient` - Value between 0 and 1
/// * `diffuse` - Value between 0 and 1
/// * `specular` - Value between 0 and 1
/// * `shininess` - Value between 10 (very large highlight) and 200 (very small highlight)
#[derive(Debug, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    /// Create a new material.
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self { color, ambient, diffuse, specular, shininess }
    }

    /// Create a material with default attributes.
    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }
    
    /// Get the materials color
    pub fn color(&self) -> &Color {
        &self.color
    }
    
    /// Set a new color
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    
    /// Get the materials ambient
    pub fn ambient(&self) -> f64 {
        self.ambient
    }
    
    /// Get the materials diffuse
    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }
    
    /// Get the materials specular
    pub fn specular(&self) -> f64 {
        self.specular
    }
    
    /// Get the materials shininess
    pub fn shininess(&self) -> f64 {
        self.shininess
    }
    
    /// Calculate the lighting for a specific material
    pub fn lighting(material: &Material, 
                    light: &PointLight, 
                    position: &Point,
                    eyev: &Vector,
                    normalv: &Vector) -> Color {
        let mut diffuse: Color = Color::new(0.0, 0.0, 0.0);
        let mut specular: Color = Color::new(0.0, 0.0, 0.0);
        let mut ambient: Color = Color::new(0.0, 0.0, 0.0);

        // Combine the surface color with the light's color/inensity
        let effective_color = *material.color() * *light.intensity();

        // Find the direction to the light source
        let mut lightv = *light.position() - *position;
        lightv.norm();
        
        // Compute the ambient contribution
        ambient = effective_color * material.ambient();

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * material.diffuse() * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let inv_lightv = -lightv;
            let reflectv = inv_lightv.reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(material.shininess);
                specular = *light.intensity() * material.specular() * factor;
            }
        }

        ambient + diffuse + specular
    }

}

#[cfg(test)]
mod test {
    use crate::materials::Material;
    use crate::canvas::color::Color;
    use crate::light::*;
    use crate::math::{
        vector::Vector,
        point::Point,
    };

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_eq!(Color::new(1.0, 1.0, 1.0), *m.color());
        assert_eq!(0.1, m.ambient());
        assert_eq!(0.9, m.diffuse());
        assert_eq!(0.9, m.specular());
        assert_eq!(200.0, m.shininess());
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let result = Material::lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let result = Material::lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let result = Material::lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(Color::new(0.7363961, 0.7363961, 0.7363961), result);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));
        let result = Material::lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(Color::new(1.636396, 1.636396, 1.636396), result);
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0));
        let result = Material::lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
    
}
