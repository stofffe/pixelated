use crate::Context;
use std::time::Instant;

const MS_AVERAGE_SAMPLED_TICKS: usize = 100;

pub(crate) struct TimeContext {
    // dt
    start_time: Instant,
    last_time: Instant,
    delta_time: f32,

    // frame time
    frame_times: [f32; MS_AVERAGE_SAMPLED_TICKS],
    frame_index: usize,
    frame_time_avg: f32,
    frame_time_sum: f32,

    time_since_start: f32,
}

impl Default for TimeContext {
    fn default() -> Self {
        let start_time = Instant::now();
        Self {
            start_time,
            last_time: start_time,
            delta_time: 0.0,

            frame_times: [0.0; MS_AVERAGE_SAMPLED_TICKS],
            frame_index: 0,
            frame_time_avg: 0.0,
            frame_time_sum: 0.0,

            time_since_start: 0.0,
        }
    }
}

impl TimeContext {
    pub(crate) fn update_time(&mut self) {
        let now = Instant::now();

        // update dt
        self.delta_time = now.duration_since(self.last_time).as_secs_f32();

        // frame time
        let (new_ms, old_ms) = (self.delta_time, self.frame_times[self.frame_index]);
        self.frame_times[self.frame_index] = new_ms;
        self.frame_index = (self.frame_index + 1) % MS_AVERAGE_SAMPLED_TICKS;
        self.frame_time_sum += new_ms - old_ms;
        self.frame_time_avg = self.frame_time_sum / MS_AVERAGE_SAMPLED_TICKS as f32;

        // time since start
        self.time_since_start = Instant::now().duration_since(self.start_time).as_secs_f32();

        self.last_time = now;
    }
}

//
// Commands
//

/// Time since the start of the application
pub fn time_since_start(ctx: &Context) -> f32 {
    ctx.time.time_since_start
}

/// Time at start of frame
pub fn current_time(ctx: &Context) -> Instant {
    ctx.time.last_time
}

/// Last frame's delta time
pub fn delta_time(ctx: &Context) -> f32 {
    ctx.time.delta_time
}

/// Frame time (seconds) averaged over recent frames
pub fn frame_time(ctx: &Context) -> f32 {
    ctx.time.frame_time_avg
}

/// Frames per second averaged over recent frames
pub fn fps(ctx: &Context) -> f32 {
    1.0 / ctx.time.frame_time_avg
}
