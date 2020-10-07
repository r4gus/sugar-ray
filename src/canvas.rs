pub mod color;

use color::Color;
use num_traits::{Float, Zero};
use std::cmp;

#[derive(Debug)]
pub struct Canvas<T: Float + Copy> {
    pixels: Vec<Vec<Color<T>>>,
}

impl<T: Float> Canvas<T> {
    /** Create a new Canvas with width and height.
     *
     * All pixels are initialized to black (0, 0, 0).
     */
    pub fn new(width: usize, height: usize) -> Self {
        Canvas { 
            pixels: vec![vec![Color::<T>::new(Zero::zero(),Zero::zero(),Zero::zero()); width]; height] 
        }
    }
    
    /** Set color for the given pixel.
     */
    pub fn write_pixel(&mut self, width: usize, height: usize, color: Color<T>) {
        assert!(height < self.pixels.len());  
        assert!(width < self.pixels[height].len());

        self.pixels[height][width] = color;
    }
    
    /** Get color of specified pixel.
     */
    pub fn pixel_at(&self, width: usize, height: usize) -> Color<T> {
        assert!(height < self.pixels.len());  
        assert!(width < self.pixels[height].len());

        self.pixels[height][width]
    }
}

impl<T: Float + cmp::PartialEq> cmp::PartialEq for Canvas<T> {
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

    #[test]
    fn creating_a_canvas() {
        let c = Canvas { 
            pixels: vec![vec![Color::<f64>::new(0.0, 0.0, 0.0); 10]; 20]
        };

        assert_eq!(c, Canvas::new(10, 20));
    }

    #[test]
    fn rows_not_equal() {
        let c = Canvas { 
            pixels: vec![vec![Color::<f64>::new(0.0, 0.0, 0.0); 10]; 19]
        };

        assert_ne!(c, Canvas::new(10, 20));
    }

    #[test]
    fn widths_not_equal() {
        let c = Canvas { 
            pixels: vec![vec![Color::<f64>::new(0.0, 0.0, 0.0); 9]; 20]
        };

        assert_ne!(c, Canvas::new(10, 20));
    }

    #[test]
    fn not_initial_color_black() {
        let c = Canvas { 
            pixels: vec![vec![Color::<f64>::new(1.0, 0.0, 0.0); 10]; 20]
        };

        assert_ne!(c, Canvas::new(10, 20));
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::<f64>::new(10, 10); 
        c.write_pixel(6, 4, Color::new(1.0, 0.0, 0.0));
        assert_eq!(Color::<f64>::new(1.0, 0.0, 0.0), c.pixels[4][6]);
    }

    #[test]
    fn pixel_at_test() {
        let mut c = Canvas::<f64>::new(10, 10); 
        c.write_pixel(6, 4, Color::new(1.0, 0.0, 0.0));
        assert_eq!(Color::<f64>::new(1.0, 0.0, 0.0), c.pixel_at(6, 4));
    }
}


