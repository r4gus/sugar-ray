use sugar_ray::math::{
    point::Point, 
    vector::Vector, 
    matrix::{Matrix, transformation::*},
};
use sugar_ray::canvas::{
    *,
    color::*,
};
use sugar_ray::ppm::*;
use std::io::prelude::*;

pub struct Clock {
    size: usize,
    canvas: Canvas,
    hour: f64,
    minute: f64,
}

impl Clock {
    pub fn new(size: usize , hour: f64, minute: f64) -> Self {
        Clock { size, canvas: Canvas::new(size, size), hour, minute }
    }
    
    pub fn draw_clock_face(&mut self) {
        let angle = std::f64::consts::PI / 6.0; // 360 deg (2*pi radians) evenly divided throudh 12
        let radius = self.size as f64 * (3.0 / 8.0); // the distance between origin and each digit
        
        // draw the origin
        self.canvas.write_pixel(self.size / 2, self.size / 2, Color::new(1.0,0.0,0.0));
        
        // draw each digit in a loop
        for i in 0..12 {
            let transform = Matrix::identity()
                                .rotate_y(i as f64 * angle) // rotate the point by i * ange radians
                                .scale(radius, 0.0, radius) // scale it outwards
                                .translate(self.size as f64 / 2.0, 0.0, self.size as f64 / 2.0); // move
                                
            let p = transform * Point::new(0.0,0.0,1.0); // apply transformation
            self.canvas.write_pixel(p.x() as usize, p.z() as usize, Color::new(1.0,0.0,0.0)); // draw
        }
    }

    pub fn draw_clockhand(&mut self) {
        // Remember everything is upside down because of the
        // internal representation of each bit.
        // Because of that some variables are negatet.
        let r = self.hour / 6.0 * std::f64::consts::PI;     // hour
        let r2 = self.minute / 30.0 * std::f64::consts::PI; // minute
        
        // big clockhand
        for i in 1..((self.size as f64 * (3.0 / 8.0)) - 6.0) as isize {
            let transform = Matrix::identity()
                                .rotate_y(-r)
                                .scale(-i as f64, 0.0, -i as f64)
                                .translate(self.size as f64 / 2.0, 0.0, self.size as f64 / 2.0);


            let p = transform * Point::new(0.0,0.0,1.0);
            self.canvas.write_pixel(p.x() as usize, p.z() as usize, Color::new(0.0,0.0,1.0));
        }
        
        // small clockhand
        for i in 1..((self.size as f64 * (3.0 / 8.0)) - 3.0) as isize {
            let transform2 = Matrix::identity()
                                .rotate_y(-r2)
                                .scale(-i as f64, 0.0, -i as f64)
                                .translate(self.size as f64 / 2.0, 0.0, self.size as f64 / 2.0);

            let p2 = transform2 * Point::new(0.0,0.0,1.0);
            self.canvas.write_pixel(p2.x() as usize, p2.z() as usize, Color::new(0.0,1.0,0.0));
        }

    }

    pub fn out(&self) -> std::io::Result<()> {
        let mut f = std::fs::File::create("images/clock.ppm")?;
        f.write_all(&self.canvas.to_ppm().into_bytes())?;

        Ok(())
    }

}
