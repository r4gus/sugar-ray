use sugar_ray::{
    math::{
        point::Point,
        vector::Vector,
    },
    shapes::Sphere,
    ray::Ray,
    canvas::{*, color::*,},
    ppm::*,
};
use std::io::prelude::*;

pub fn render_sphere(canvas_size: usize) -> std::io::Result<()> {
    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let s = Sphere::new(); // New sphere at origin (0, 0, 0)
    let color = Color::new(8.0, 5.0, 8.0); // Red
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;  // Distance from the origin to the wall behind the sphere
    let wall_size = 7.0; // width and heigth of the wall
    let pixel_size = wall_size / canvas_size as f64; // size of a pixel on the wall relative to the canv.
    let half = wall_size / 2.0;

    for y in 0..canvas_size {
        // Origin on the canvas is the top left corner.
        // Origin on the wall is the middle (on both x and y axis).
        // So we add half to the y coordinate and then move downwards on the wall.
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_size {
            // Same as for world y.
            // We start at the left corner of our wall and the move to the right, up to the right
            // corner.
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);
            let mut v = position - ray_origin;
            v.norm();
            let ray = Ray::new(ray_origin, v);

            if let Some(_) = ray.intersect_sphere(&s) {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut f = std::fs::File::create("images/sphere.ppm")?;
    f.write_all(&canvas.to_ppm().into_bytes())?;

    Ok(())
}
