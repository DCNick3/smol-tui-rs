use crate::{FixedFrameAccessor, FrameAccessor};

pub trait Widget<T> {
    fn render(&self, frame: &mut FrameAccessor<T>);
}

pub trait FixedWidget<T, const W: usize, const H: usize> {
    fn render(&self, frame: &mut FixedFrameAccessor<T, W, H>);
}

// adapter to use erased widget with a fixed frame
impl<T, WIDGET, const W: usize, const H: usize> FixedWidget<T, W, H> for WIDGET
where
    WIDGET: Widget<T>,
{
    fn render(&self, frame: &mut FixedFrameAccessor<T, W, H>) {
        self.render(&mut frame.into())
    }
}
