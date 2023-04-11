use crate::{input::InputContext, render::RenderContext, time::TimeContext};

pub struct Context {
    pub render: RenderContext,
    pub(crate) time: TimeContext,
    pub input: InputContext,
}
