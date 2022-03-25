use core::{cmp::max, marker::PhantomData};

use smol_tui_derive::fixed_widget_adapter;

use crate::{
    widget::Widget, DefaultFill, FixedFrameAccessor, FixedWidget, FrameAccessor, FrameAccessorTrait,
};

// Fills all the provided frame with the same char passed through state
#[derive(Default)]
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

// Shows a one-character spinner (TODO: maybe allow the symbols to be changed?)
#[derive(Default)]
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

// left-aligned label
#[derive(Default)]
pub struct LAlignedLabel<T>
where
    T: Copy,
{
    phantom: PhantomData<T>,
}

// center-aligned label
#[derive(Default)]
pub struct CAlignedLabel<T>
where
    T: Copy,
{
    phantom: PhantomData<T>,
}

// right-aligned label
#[derive(Default)]
pub struct RAlignedLabel<T>
where
    T: Copy,
{
    phantom: PhantomData<T>,
}

#[fixed_widget_adapter]
impl<T> Widget<T> for LAlignedLabel<T>
where
    T: Copy,
    T: DefaultFill,
{
    type State = [T];

    fn render(&self, state: &[T], frame: &mut FrameAccessor<T>, _tick: u32) {
        frame.fill(T::default());
        for (i, v) in (0..frame.width()).zip(state) {
            frame[(i, 0)] = *v;
        }
    }
}

#[fixed_widget_adapter]
impl<T> Widget<T> for CAlignedLabel<T>
where
    T: Copy,
    T: DefaultFill,
{
    type State = [T];

    fn render(&self, state: &[T], frame: &mut FrameAccessor<T>, _tick: u32) {
        frame.fill(T::default());
        // need a bit of maths here though

        // how many positions frame is wider that the rendered string?
        let diff = frame.width() as isize - state.len() as isize;
        // put half of that different before the text, the other will be placed after
        // if the diff is negative - put it in the beginning
        let offset = (max(0, diff) / 2) as usize;

        for (i, v) in (offset..frame.width()).zip(state) {
            frame[(i, 0)] = *v;
        }
    }
}

#[fixed_widget_adapter]
impl<T> Widget<T> for RAlignedLabel<T>
where
    T: Copy,
    T: DefaultFill,
{
    type State = [T];

    fn render(&self, state: &[T], frame: &mut FrameAccessor<T>, _tick: u32) {
        frame.fill(T::default());
        // magic, just iterate from the other direction =)
        for (i, v) in (0..frame.width()).rev().zip(state.iter().rev()) {
            frame[(i, 0)] = *v;
        }
    }
}

// OMG this is scary AF
// it is an adapter widget that subframes the frame when accessed
// smol_tui_scene macro wraps all widgets in this to implement positioning
#[derive(Default)]
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
