use sdl2::pixels::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CelestialBodyType {
    Star,
    Planet,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CelestialBody {
    pub b_type: CelestialBodyType,
    pub radius: f64,
    pub color: Color,
} 
