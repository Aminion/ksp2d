use glam::dvec2;
use legion::{world::SubWorld, *};

use crate::{ksp2d::components::newton_body::NewtonBody, Dt};

const G: f64 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

#[system]
#[write_component(NewtonBody)]
pub fn celestial_body(world: &mut SubWorld, #[resource] dt: &Dt) {
    let mut query = <&mut NewtonBody>::query();
    let mut r: Vec<&mut NewtonBody> = query.iter_mut(world).collect();
    n_body_iter(&mut r, dt);
}

fn n_body_iter(objs: &mut [&mut NewtonBody], dt: &Dt) {
    for i in 0..objs.len() {
        let mut a = dvec2(0.0, 0.0);
        for j in 0..objs.len() {
            if i == j {
                continue;
            }

            let dist = objs[j].pos - objs[i].pos;
            let dist_sq = dist.dot(dist);
            let dist_cu = dist_sq.sqrt() * dist_sq;
            let f = G * objs[i].mass * objs[j].mass / dist_sq;
            a += f * dist / dist_cu;
        }
        objs[i].acc += a;
    }

    for o in objs.iter_mut() {
        let dv = o.acc * dt.0;
        o.vel += dv;
        let next_pos = 2.0 * o.pos - o.prev_pos + dv * dt.0;
        o.prev_pos = o.pos;
        o.pos = next_pos;
        o.update_a(dt)
    }
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
