pub mod color;

use color::Color;
use super::ppm::{Ppm, PpmColor};

use std::cmp;


#[derive(Debug)]
pub struct Canvas {
    pixels: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl Canvas {
    /** Create a new Canvas with width and height.
     *
     * All pixels are initialized to black (0, 0, 0).
     */
    pub fn new(width: usize, height: usize) -> Self {
        Canvas { 
            pixels: vec![vec![Color::new(0.0,0.0,0.0); width]; height],
            width,
            height
        }
    }
    
    /** Set color for the given pixel.
     */
    pub fn write_pixel(&mut self, width: usize, height: usize, color: Color) {
        assert!(height < self.pixels.len());  
        assert!(width < self.pixels[height].len());

        self.pixels[height][width] = color;
    }
    
    /** Get color of specified pixel.
     */
    pub fn pixel_at(&self, width: usize, height: usize) -> Color {
        assert!(height < self.pixels.len());  
        assert!(width < self.pixels[height].len());

        self.pixels[height][width]
    }
}

impl Ppm for Canvas {
    fn to_ppm(&self) -> String {
        const PIXELS_PER_LINE: u32 = 5;
        let mut pixelcount: u32 = PIXELS_PER_LINE;

        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        
        for rows in &self.pixels {
            for pixel in rows {
                
                pixelcount -= 1;
                ppm.push_str(&pixel.to_ppm_color()); // convert pixel to a (r, g, b) color string

                if pixelcount == 0 {
                    pixelcount = PIXELS_PER_LINE;
                    ppm.push_str("\n");
                } else {
                    ppm.push_str(" ");
                }
            }
        }
        
        // last element hast to be a new line
        ppm.pop();  // removes either newline or space
        ppm.push('\n');

        ppm
    }
}

impl cmp::PartialEq for Canvas {
    fn eq(&self, other: &Self) -> bool {
        // Number of rows doesn't match
        if self.pixels.len() != other.pixels.len() {
            return false;
        }
        
        // Iterate over rows
        for i in 0..self.pixels.len() {
            if self.pixels[i].len() != other.pixels[i].len() {
                // Rows not of the same size
                return false;
            }
            
            // Iterate over each pixel of a row
            for j in 0..self.pixels[i].len() {
                if self.pixels[i][j] != other.pixels[i][j] {
                    // Colors don't match
                    return false;
                }   
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::{
        Canvas,
        color::Color
    };
    use crate::ppm::Ppm;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas { 
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); 10]; 20],
            width: 10,
            height: 20
        };

        assert_eq!(c, Canvas::new(10, 20));
    }

    #[test]
    fn rows_not_equal() {
        let c = Canvas { 
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); 10]; 19],
            width: 10,
            height: 19
        };

        assert_ne!(c, Canvas::new(10, 20));
    }

    #[test]
    fn widths_not_equal() {
        let c = Canvas { 
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); 9]; 20],
            width: 9,
            height: 20,
        };

        assert_ne!(c, Canvas::new(10, 20));
    }

    #[test]
    fn not_initial_color_black() {
        let c = Canvas { 
            pixels: vec![vec![Color::new(1.0, 0.0, 0.0); 10]; 20],
            width: 10,
            height: 20
        };

        assert_ne!(c, Canvas::new(10, 20));
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 10); 
        c.write_pixel(6, 4, Color::new(1.0, 0.0, 0.0));
        assert_eq!(Color::new(1.0, 0.0, 0.0), c.pixels[4][6]);
    }

    #[test]
    fn pixel_at_test() {
        let mut c = Canvas::new(10, 10); 
        c.write_pixel(6, 4, Color::new(1.0, 0.0, 0.0));
        assert_eq!(Color::new(1.0, 0.0, 0.0), c.pixel_at(6, 4));
    }

    #[test]
    fn constructing_the_ppm_header() {
        let expected = String::from("P3\n5 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n"); 
        assert_eq!(expected, Canvas::new(5, 3).to_ppm());
    }
    
    #[test]
    fn constructing_the_pixel_data() {
        let expected = String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"); 
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));

        assert_eq!(expected, canvas.to_ppm());
    }

    #[test]
    fn ends_with_new_line() {
        assert_eq!('\n', Canvas::new(5,3).to_ppm().pop().unwrap()); 
    }
}


