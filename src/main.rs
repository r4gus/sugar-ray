pub mod examples;

use examples::{
    projectile, // fire a projectile over a canvas
    clock::Clock,
};

fn main() -> std::io::Result<()> {
    //projectile::fire()   
    
    let mut c = Clock::new(50, 3.5, 30.0);
    c.draw_clock_face();
    c.draw_clockhand();
    c.out()
}
