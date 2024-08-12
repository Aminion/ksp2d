use glam::dvec2;
use legion::{world::SubWorld, *};

use crate::{ksp2d::components::celestial_body::Obj, Dt};

const G: f64 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

#[system]
#[write_component(Obj)]
pub fn celestial_body(world: &mut SubWorld, #[resource] dt: &Dt) {
    let mut query = <&mut Obj>::query();
    let mut r: Vec<&mut Obj> = query.iter_mut(world).collect();
    n_body_iter(&mut r, dt.0);
}

fn n_body_iter(objs: &mut Vec<&mut Obj>, dt: f64) {
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
        let next_pos = 2.0 * o.pos - o.prev_pos + o.acc * dt.exp2();
        o.prev_pos = o.pos;
        o.pos = next_pos;
    }
}

pub fn calculate_energy(particles: &Vec<&mut Obj>) -> f64 {
    let mut ke = 0.0;
    let mut pe = 0.0;

    for i in 0..particles.len() {
        let particle = &particles[i];
        ke += 0.5 * particle.mass * particle.vel.dot(particle.vel);
        for j in (i + 1)..particles.len() {
            let other = &particles[j];
            let d = other.pos - particle.pos;
            let r_squared = d.dot(d);
            pe -= G * particle.mass * other.mass / r_squared.sqrt();
        }
    }

    ke + pe
}