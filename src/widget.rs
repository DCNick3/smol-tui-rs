use crate::{FixedFrameAccessor, FrameAccessor};

pub trait Widget<T> {
    type State;

    fn render(&self, state: &Self::State, frame: &mut FrameAccessor<T>, tick: u32);
}

pub trait FixedWidget<T, const W: usize, const H: usize> {
    type State;

    fn render_fixed(&self, state: &Self::State, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32);
}

// // adapter to allow rendering both fixed & size-erased widget with same API
// trait WidgetRenderer<T, WIDGET, const W: usize, const H: usize> {
//     fn render(widget: &WIDGET, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32);
// }

// impl<T, WIDGET: Widget<T>, const W: usize, const H: usize> WidgetRenderer<T, WIDGET, W, H> for WIDGET
// {
//     fn render(widget: &WIDGET, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32) {
//         widget.render(frame, tick)
//     }
// }

// impl<T, WIDGET: FixedWidget<T, W, H>, const W: usize, const H: usize> WidgetRenderer<T, WIDGET, W, H> for WIDGET
// {
//     fn render(widget: &WIDGET, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32) {
//         widget.render(frame, tick)
//     }
// }

/*impl<T, WIDGET, const W: usize, const H: usize> FixedWidget<T, W, H> for WIDGET
where
    WIDGET: Widget<T>,
{
    fn render(&self, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32) {
        self.render(&mut frame.into(), tick)
    }
}*/
