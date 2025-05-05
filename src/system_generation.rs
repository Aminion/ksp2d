use std::f64::consts::PI;

use glam::{dvec2, DVec2};
use rand::Rng;
use sdl2::pixels::Color;

use crate::ksp2d::components::{
    celestial_body::{CelestialBody, CelestialBodyType},
    newton_body::NewtonBody,
};

pub fn get_system(system_radius: f64) -> Vec<(CelestialBody, NewtonBody)> {
    let system_center = dvec2(system_radius, system_radius);

    let planet_range = 0.01..=0.04;
    let planet_density_range = 1330.0..=5420.0;
    let planet_mass_range = 0.330e24..=1898.6e24;
    let interval_range = 0.05..=0.1;

    let star_radius = 6.957e8 / 8.0;
    let star_mass = 1.988416e30;

    let star = (
        CelestialBody {
            b_type: CelestialBodyType::Star,
            color: Color::YELLOW,
            radius: star_radius,
        },
        NewtonBody {
            angle: DVec2::Y,
            angular_vel: 1.0,
            mass: star_mass,
            pos: system_center,
            vel: DVec2::ZERO,
            acc: DVec2::ZERO,
        },
    );
    let mut system: Vec<(CelestialBody, NewtonBody)> = Vec::new();
    system.push(star);

    let mut rng = rand::rng();
    let mut cursor = star_radius;
    let limit =
        system_radius - (system_radius * planet_range.end() + system_radius * interval_range.end());
    while cursor <= limit {
        let interval = system_radius * rng.random_range(interval_range.clone());
        cursor += interval;
        let mass = rng.random_range(planet_mass_range.clone());
        let dencity = rng.random_range(planet_density_range.clone());
        let p_r = calculate_radius(mass, dencity);
        cursor += p_r;
        let angle = rng.random_range(0.0..2.0 * std::f64::consts::PI);
        let position = cursor * DVec2::from_angle(angle);
        let orbital_speed =
            (physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION * star.1.mass / cursor).sqrt();

        let velocity = DVec2::new(-position.y, position.x).normalize() * orbital_speed;

        let planet = (
            CelestialBody {
                b_type: CelestialBodyType::Planet,
                color: Color::GREEN,
                radius: p_r,
            },
            NewtonBody {
                angle: DVec2::Y,
                angular_vel: angle,
                mass: mass,
                pos: system_center + position,
                vel: velocity,
                acc: DVec2::ZERO,
            },
        );
        system.push(planet);
        cursor += p_r;
    }
    system
}

fn calculate_radius(mass: f64, density: f64) -> f64 {
    let volume = mass / density;
    let radius_cubed = (3.0 * volume) / (4.0 * PI);
    radius_cubed.cbrt()
}
