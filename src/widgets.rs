use core::marker::PhantomData;

use smol_tui_derive::fixed_widget_adapter;

use crate::{widget::Widget, FrameAccessor, FrameAccessorTrait, FixedWidget, FixedFrameAccessor};

pub struct Filler<T>
where
    T: Copy,
{
    filler: T,
}

impl<T> Filler<T>
where
    T: Copy,
{
    pub fn new(filler: T) -> Self {
        Filler { filler }
    }
}

// this attribute is required to be able to render size-erased widgets
#[fixed_widget_adapter]
impl<T> Widget<T> for Filler<T>
where
    T: Copy,
{
    fn render(&self, frame: &mut FrameAccessor<T>, _tick: u32) {
        frame.fill(self.filler)
    }
}

pub struct Spinner<T>
where
    T: Copy,
    T: From<char>,
{
    phantom: PhantomData<T>
}

impl<T> FixedWidget<T, 1, 1> for Spinner<T>
    where
        T: Copy,
        T: From<char>,
{
    fn render_fixed(&self, frame: &mut FixedFrameAccessor<T, 1, 1>, tick: u32) {
        static FRAMES: &'static [char] = &['.', 'o', 'O', 'o', '.'];
        let val = FRAMES[tick as usize % FRAMES.len()];

        frame.fill(val)
    }
}