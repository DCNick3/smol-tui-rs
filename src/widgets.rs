use core::marker::PhantomData;

use smol_tui_derive::fixed_widget_adapter;

use crate::{widget::Widget, FixedFrameAccessor, FixedWidget, FrameAccessor, FrameAccessorTrait};

pub struct Filler<T>
where
    T: Copy,
{
    phantom: PhantomData<T>,
}

impl<T> Filler<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Filler {
            phantom: PhantomData {},
        }
    }
}

// this attribute is required to be able to render size-erased widgets
#[fixed_widget_adapter]
impl<T> Widget<T> for Filler<T>
where
    T: Copy,
{
    type State = T;

    fn render(&self, state: &T, frame: &mut FrameAccessor<T>, _tick: u32) {
        frame.fill(*state)
    }
}

pub struct Spinner<T>
where
    T: Copy,
    T: From<char>,
{
    phantom: PhantomData<T>,
}

impl<T> Spinner<T>
where
    T: Copy,
    T: From<char>,
{
    pub fn new() -> Self {
        Self {
            phantom: PhantomData {},
        }
    }
}

impl<T> FixedWidget<T, 1, 1> for Spinner<T>
where
    T: Copy,
    T: From<char>,
{
    type State = ();

    fn render_fixed(&self, _state: &(), frame: &mut FixedFrameAccessor<T, 1, 1>, tick: u32) {
        static FRAMES: &'static [char] = &['.', 'o', 'O', 'o', '.']; // TODO this is not the best way to handle it... I guess
        let val = FRAMES[(tick / 4) as usize % FRAMES.len()];

        frame.fill(val)
    }
}

// OMG this is scary AF
pub struct Subframer<
    T,
    Inner,
    const W: usize,
    const H: usize,
    const X: usize,
    const Y: usize,
    const SUB_W: usize,
    const SUB_H: usize,
> where
    T: Copy,
    T: From<char>,
    Inner: FixedWidget<T, SUB_W, SUB_H>,
{
    inner: Inner,
    phantom: PhantomData<T>,
}

impl<
        T,
        Inner,
        const W: usize,
        const H: usize,
        const X: usize,
        const Y: usize,
        const SUB_W: usize,
        const SUB_H: usize,
    > Subframer<T, Inner, W, H, X, Y, SUB_W, SUB_H>
where
    T: Copy,
    T: From<char>,
    Inner: FixedWidget<T, SUB_W, SUB_H>,
{
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            phantom: PhantomData {},
        }
    }
}

impl<
        T,
        Inner,
        const W: usize,
        const H: usize,
        const X: usize,
        const Y: usize,
        const SUB_W: usize,
        const SUB_H: usize,
    > FixedWidget<T, W, H> for Subframer<T, Inner, W, H, X, Y, SUB_W, SUB_H>
where
    T: Copy,
    T: From<char>,
    Inner: FixedWidget<T, SUB_W, SUB_H>,
{
    type State = Inner::State;

    fn render_fixed(
        &self,
        state: &Self::State,
        frame: &mut FixedFrameAccessor<T, W, H>,
        tick: u32,
    ) {
        self.inner
            .render_fixed(state, &mut frame.subframe::<X, Y, SUB_W, SUB_H>(), tick)
    }
}
