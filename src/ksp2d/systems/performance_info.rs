use crate::{Dt, PerformanceInfo};
use legion::system;
use std::time::{Duration, Instant};

const PERIOD: Duration = Duration::from_secs(1);

#[system]
pub fn update_info(#[resource] dt: &Dt, #[resource] info: &mut PerformanceInfo) {
    let elapsed = info.update_timer.elapsed();
    if elapsed >= PERIOD {
        info.fps = dt.0.recip() as u64;
        info.update_timer = Instant::now()
    }
}
