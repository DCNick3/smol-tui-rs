use crate::{widget::Widget, FrameAccessor, FrameAccessorTrait};

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

impl<T> Widget<T> for Filler<T>
where
    T: Copy,
{
    fn render(&self, frame: &mut FrameAccessor<T>) {
        frame.fill(self.filler)
    }
}
