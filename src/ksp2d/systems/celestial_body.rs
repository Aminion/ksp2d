use glam::dvec2;
use legion::{world::SubWorld, *};
use log::info;

use crate::{ksp2d::components::celestial_body::Obj, Dt};

#[system]
#[read_component(Obj)]
pub fn celestial_body(world: &mut SubWorld, #[resource] dt: &Dt) {
    info!("LLLLLLLLLLLLL");
    let mut query = <&mut Obj>::query();
    info!("LLLLLLLLLLLLL");
    let mut r: Vec<&mut Obj> = query.iter_mut(world).collect();
    info!("LLLLLLLLLLLLL");
    n_body_iter(&mut r, dt.0)
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
        o.vel += o.acc * dt;
        o.pos += o.vel * dt;
    }
}

const G: f64 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;
