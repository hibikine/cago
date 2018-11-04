extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;

use nphysics2d::world::World;

fn main() {
    println!("Hello, world!");
}

#[no_mangle]
pub fn run() {
    let world = World::new();
}
