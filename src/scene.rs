use crate::frame_accessor::FixedFrameAccessor;

pub trait Scene<T, const W: usize, const H: usize> {
    fn render(&self, frame: &mut FixedFrameAccessor<T, W, H>, tick: u32);
}

pub trait DefaultFill: Copy {
    fn default() -> Self;
}

impl DefaultFill for u8 {
    fn default() -> Self {
        ' ' as u8
    }
}
