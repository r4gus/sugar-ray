pub mod examples;

use examples::{
    projectile, // fire a projectile over a canvas
    clock::Clock,
    sphere,
};

fn main() -> std::io::Result<()> {
    // Porjectile Demo
    //projectile::fire()   
   
    /* Clock Demo
    let mut c = Clock::new(50, 3.5, 30.0);
    c.draw_clock_face();
    c.draw_clockhand();
    c.out()
    */

    /* Sphere Demo */
    sphere::render_sphere(1024)
}
