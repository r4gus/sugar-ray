use sugar_ray::math::{Point, Vector};

use std::thread::sleep;
use std::time::Duration;

struct Projectile {
    pub position: Point<f64>,
    pub velocity: Vector<f64>,
}

struct Environment {
    pub gravity: Vector<f64>,
    pub wind: Vector<f64>,
}

fn tick<'a>(env:&Environment, proj: &'a mut Projectile) -> &'a Projectile {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
    proj
}

macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {
        println!("{:?} :: {:?}", stringify!($left), stringify!($right));
    }
}

fn main() {
    /*
    println!("Setting up Environment...");
    let env = Environment { gravity: Vector::new(0.0,-0.1,0.0), wind: Vector::new(-0.01, 0.0, 0.0) };
    println!("Loading projectile...");
    let mut proj = Projectile { position: Point::new(0.0,1.0,0.0), velocity: Vector::new(1.0, 1.0, 0.0).norm_cpy() };
    
    let ten_millis = Duration::from_millis(100);

    while proj.position.y > 0.0  {
        tick(&env, &mut proj);
        println!("Position: x -> {}, y -> {}, z -> {}", proj.position.x, proj.position.y, proj.position.z);
        sleep(ten_millis);
    }
    */

    assert_float_eq!(0.5_f32, 0.7_f64);
}
