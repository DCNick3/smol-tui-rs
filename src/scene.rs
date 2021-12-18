use crate::frame_accessor::FixedFrameAccessor;

pub trait Scene<T, const W: usize, const H: usize> {
    fn render(&self, frame: &mut FixedFrameAccessor<T, W, H>);
}
