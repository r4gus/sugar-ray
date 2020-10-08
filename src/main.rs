use sugar_ray::math::{point::Point, vector::Vector};
use sugar_ray::canvas::{
    *,
    color::*,
};
use sugar_ray::ppm::*;

use std::thread::sleep;
use std::time::Duration;
use std::io::prelude::*;

struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

fn tick<'a>(env:&Environment, proj: &'a mut Projectile) -> &'a Projectile {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
    proj
}

fn main() -> std::io::Result<()> {
    println!("Setting up Environment...");
    let env = Environment { gravity: Vector::new(0.0,-0.1,0.0), wind: Vector::new(-0.01, 0.0, 0.0) };
    println!("Loading projectile...");
    let mut proj = Projectile { position: Point::new(0.0,1.0,0.0), velocity: Vector::new(1.0, 1.8, 0.0).norm_cpy() * 11.25 };
    println!("Preparing canvas...");
    let mut canvas = Canvas::<f32>::new(900, 550);
    

    while proj.position.y() > 0.0  {
        tick(&env, &mut proj);
        canvas.write_pixel(proj.position.x() as usize, 549 - (proj.position.y() as usize), Color::new(1.0, 0.0, 0.0));
    }

    let mut f = std::fs::File::create("canvas.ppm")?;
    f.write_all(&canvas.to_ppm().into_bytes())?;
    Ok(())
}
