use crate::{input::InputContext, render::RenderContext, time::TimeContext};

/// Holds all the neccesary state for running the engine
///
/// Sent with each command
pub struct Context {
    pub(crate) render: RenderContext,
    pub(crate) time: TimeContext,
    pub(crate) input: InputContext,
}
