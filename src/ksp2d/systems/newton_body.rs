use glam::DVec2;
use legion::{world::SubWorld, *};

use crate::{ksp2d::components::newton_body::NewtonBody, Dt};

const G: f64 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

#[system]
#[write_component(NewtonBody)]
pub fn celestial_body(world: &mut SubWorld, #[resource] dt: &Dt) {
    let mut query = <&mut NewtonBody>::query();
    let mut r: Vec<&mut NewtonBody> = query.iter_mut(world).collect();
    n_body_iter(&mut r, dt);//&Dt(3600.0)
}

fn n_body_iter(objs: &mut [&mut NewtonBody], dt: &Dt) {
    let dt_f = dt.0;
    let num_bodies = objs.len();

    for i in 0..num_bodies {
        objs[i].pos += objs[i].vel * dt_f + 0.5 * objs[i].acc * dt_f * dt_f;
    }

    let mut forces = vec![vec![DVec2::ZERO; num_bodies]; num_bodies];

    for i in 0..num_bodies {
        for j in (i + 1)..num_bodies {
            let force_ij = gravitational_force(objs[i], objs[j]);
            forces[i][j] = force_ij;
            forces[j][i] = -force_ij;
        }
    }
    let mut new_accelerations = vec![DVec2::ZERO; num_bodies];
    for i in 0..num_bodies {
        let mut net_force = DVec2::ZERO;
        for j in 0..num_bodies {
            if i != j {
                net_force += forces[i][j];
            }
        }
        new_accelerations[i] = net_force / objs[i].mass;
    }

    for i in 0..num_bodies {
        objs[i].vel += 0.5 * (objs[i].acc + new_accelerations[i]) * dt_f;
        objs[i].acc = new_accelerations[i];
        objs[i].update_a(dt);
    }

    // println!("{}", calculate_energy(objs))
}

pub fn gravitational_force(body1: &NewtonBody, body2: &NewtonBody) -> DVec2 {
    let r_vec = body2.pos - body1.pos;
    let r_sq = r_vec.length_squared();
    let force_magnitude = G * body1.mass * body2.mass / r_sq;
    let force_direction = r_vec.normalize();

    force_magnitude * force_direction
}

pub fn calculate_energy(bodies: &[&mut NewtonBody]) -> f64 {
    let mut ke = 0.0;
    let mut pe = 0.0;

    let num_bodies = bodies.len();

    for body in bodies {
        ke += 0.5 * body.mass * body.vel.length_squared();
    }

    for i in 0..num_bodies {
        for j in (i + 1)..num_bodies {
            let r_vec = bodies[j].pos - bodies[i].pos;
            let distance = r_vec.length();
            if distance > f64::EPSILON * 100.0 {
                pe -= G * bodies[i].mass * bodies[j].mass / distance;
            }
        }
    }

    ke + pe
}
