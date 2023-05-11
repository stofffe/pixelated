use std::time;

use crate::context::Context;

/// Returns current time
pub fn current_time(ctx: &Context) -> time::SystemTime {
    ctx.time.current_time
}
