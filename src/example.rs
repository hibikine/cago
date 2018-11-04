extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;

use na::Vector2;
use ncollide2d::math::Isometry;
use ncollide2d::shape::Cuboid;
use ncollide2d::shape::ShapeHandle;
use nphysics2d::volumetric::Volumetric;
use nphysics2d::world::World;

pub fn main() {
    let mut world = World::<f64>::new();
    world.set_gravity(Vector2::y() * -9.81);
    let cuboid = ShapeHandle::new(Cuboid::new(Vector2::new(1.0, 2.0)));
    let local_inertia = cuboid.inertia(1.0);
    let local_center_of_mass = cuboid.center_of_mass();
    let rigid_body_handle = world.add_rigid_body(
        Isometry::new(Vector2::x() * 2.0, na::zero()),
        local_inertia,
        local_center_of_mass,
    );
    println(rigid_body_handle)
}
