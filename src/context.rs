use crate::{input::InputContext, render::RenderContext, time::TimeContext};

pub struct Context {
    pub(crate) render: RenderContext,
    pub(crate) time: TimeContext,
    pub(crate) input: InputContext,
}
