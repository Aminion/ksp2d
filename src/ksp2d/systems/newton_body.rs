use glam::DVec2;
use legion::{world::SubWorld, *};

use crate::{ksp2d::components::newton_body::NewtonBody, Dt};

const G: f64 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

#[system]
#[write_component(NewtonBody)]
pub fn celestial_body(world: &mut SubWorld, #[resource] dt: &Dt) {
    let mut query = <&mut NewtonBody>::query();
    let mut r: Vec<&mut NewtonBody> = query.iter_mut(world).collect();
    n_body_iter(&mut r, &Dt(dt.0 * 1000000.0));
}

fn n_body_iter(objs: &mut [&mut NewtonBody], dt: &Dt) {
    for i in 0..objs.len() {
        let mut f_a = DVec2::ZERO;
        for j in 0..objs.len() {
            if i == j {
                continue;
            }
            f_a += gravitational_force(objs[i], objs[j]);
        }
        objs[i].acc = f_a / objs[i].mass;
    }

    for o in objs.iter_mut() {
        o.vel += o.acc * dt.0;
        o.pos += o.vel * dt.0;
        o.update_a(dt);
    }
}

// Function to calculate the gravitational force between two bodies
pub fn gravitational_force(body1: &NewtonBody, body2: &NewtonBody) -> DVec2 {
    let r_vec = body2.pos - body1.pos;
    let r_sq = r_vec.length_squared();
    let force_magnitude = G * body1.mass * body2.mass / r_sq;
    let force_direction = r_vec.normalize();

    force_magnitude * force_direction
}

pub fn calculate_energy(particles: &[&mut NewtonBody]) -> f64 {
    let mut ke = 0.0;
    let mut pe = 0.0;

    for i in 0..particles.len() {
        let particle = &particles[i];
        ke += 0.5 * particle.mass * particle.vel.dot(particle.vel);
        for other_particle in particles.iter().skip(i + 1) {
            let d = other_particle.pos - particle.pos;
            let r_squared = d.dot(d);
            pe -= G * particle.mass * other_particle.mass / r_squared.sqrt();
        }
    }

    ke + pe
}
