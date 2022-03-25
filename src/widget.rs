use crate::{FixedFrameAccessor, FrameAccessor};

pub trait Widget<T> {
    type State: ?Sized;

    fn render(&self, state: &Self::State, frame: &mut FrameAccessor<T>, tick: u32);
}

pub trait FixedWidget<T, const W: usize, const H: usize> {
    type State: ?Sized;

    fn render_fixed(&self, state: &Self::State, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32);
}
