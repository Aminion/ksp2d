use crate::{Dt, FrameDuration};
use legion::system;
use std::time::{Duration, Instant};

const PERIOD: Duration = Duration::from_secs(1);

pub struct PerformanceInfo {
    pub fps: u64,
    pub frame_time: u64,
    pub update_timer: Instant,
}

#[system]
pub fn update_info(
    #[resource] dt: &Dt,
    #[resource] ft: &FrameDuration,
    #[resource] info: &mut PerformanceInfo,
) {
    let elapsed = info.update_timer.elapsed();
    if elapsed >= PERIOD {
        info.fps = dt.0.recip() as u64;
        info.update_timer = Instant::now();
        info.frame_time = ft.0.as_micros() as u64;
    }
}
