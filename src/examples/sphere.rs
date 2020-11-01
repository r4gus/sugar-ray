use sugar_ray::{
    math::{
        point::Point,
        vector::Vector,
    },
    shapes::Sphere,
    ray::{Ray, intersection::{Intersection, Intersections}},
    canvas::{*, color::*,},
    ppm::*,
    materials::*,
    light::*,
};
use std::io::prelude::*;

pub fn render_sphere(canvas_size: usize) -> std::io::Result<()> {
    let mut canvas = Canvas::new(canvas_size, canvas_size);

    // SPHERE
    let mut s = Sphere::new(); // New sphere at origin (0, 0, 0)
    s.set_material_color(Color::new(1.0, 0.2, 1.0));
    
    // LIGHT SOURCE
    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_color, light_position);

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
            let mut v = position - ray_origin; // Ray direction
            v.norm();
            let ray = Ray::new(ray_origin, v);

            if let Some(xs) = ray.intersect_sphere(&s) {
                if let Some(mut hit) = xs.hit() {

                    let p = ray.position(hit.t());
                    let n = hit.obj().normal_at(p.clone());
                    let eye = *ray.direction() * (-1.0);

                    canvas.write_pixel(x, y, Material::lighting(hit.obj().get_material(), &light, &p, &eye, &n));
                } else {
                    canvas.write_pixel(x, y, Color::new(0.0, 0.0, 0.0));
                }
                
            }
        }
    }

    let mut f = std::fs::File::create("images/sphere.ppm")?;
    f.write_all(&canvas.to_ppm().into_bytes())?;

    Ok(())
}
