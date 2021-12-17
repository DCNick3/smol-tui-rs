use core::marker::PhantomData;

use crate::{FixedFrameAccessor, FrameAccessor};

pub trait Widget<T> {
    fn render(&self, frame: &mut FrameAccessor<T>);
}

pub trait FixedWidget<T, const W: usize, const H: usize> {
    fn render(&self, frame: &mut FixedFrameAccessor<T, W, H>);
}

pub struct FixedWidgetAdapter<T, WType, const W: usize, const H: usize>(WType, PhantomData<T>)
where
    WType: Widget<T>;

impl<T, WType, const W: usize, const H: usize> FixedWidget<T, W, H>
    for FixedWidgetAdapter<T, WType, W, H>
where
    WType: Widget<T>,
{
    fn render(&self, frame: &mut FixedFrameAccessor<T, W, H>) {
        frame.with_erased_size(|frame| self.0.render(frame))
    }
}
