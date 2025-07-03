use legion::{world::SubWorld, *};
use uom::si::{f64::Velocity, velocity::meter_per_second};

use crate::ksp2d::components::{
    closest_celestial_body::ClosestCelestialBody, flight_info::FlightInfo, newton_body::NewtonBody,
    rocket::Rocket,
};
#[system(for_each)]
#[write_component(FlightInfo)]
pub fn flight_info(
    world: &SubWorld,
    rocket: &Rocket,
    n_body: &NewtonBody,
    ccb: &ClosestCelestialBody,
    info: &mut FlightInfo,
) {
    info.delta = Velocity::new::<meter_per_second>(n_body.vel.length());
}
