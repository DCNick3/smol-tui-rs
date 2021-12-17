use core::ops::{Index, IndexMut};

// SAFETY: width & height returns the same value for all lifetime of an object
pub unsafe trait FrameAccessorTrait {
    type Element;

    fn size(&self) -> (usize, usize);
    fn width(&self) -> usize {
        self.size().0
    }
    fn height(&self) -> usize {
        self.size().1
    }

    fn data(&self) -> &[Self::Element];
    fn data_mut(&mut self) -> &mut [Self::Element];

    // SAFETY: x < width() && y < height()
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element;
    // SAFETY: x < width() && y < height()
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Element;

    fn get(&self, x: usize, y: usize) -> Option<&Self::Element> {
        if x < self.width() && y < self.height() {
            // SAFETY: x & y are checked to be in bounds
            unsafe { Some(self.get_unchecked(x, y)) }
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Element> {
        if x < self.width() && y < self.height() {
            // SAFETY: x & y are checked to be in bounds
            unsafe { Some(self.get_unchecked_mut(x, y)) }
        } else {
            None
        }
    }

    fn index(&self, index: (usize, usize)) -> &Self::Element {
        self.get(index.0, index.1)
            .unwrap_or_else(|| panic!("frame access out of bounds"))
    }

    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Element {
        self.get_mut(index.0, index.1)
            .unwrap_or_else(|| panic!("frame access out of bounds"))
    }

    fn fill<F>(&mut self, filler: F)
    where
        F: Into<Self::Element> + Copy,
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                // SAFETY: x & y are in bounds by construction
                let p = unsafe { self.get_unchecked_mut(x, y) };
                *p = filler.into();
            }
        }
    }
}

// A frame accessor with no information about frame size known at compile time (it's erased from it's type)
pub struct FrameAccessor<'a, T> {
    data: &'a mut [T],
    width: usize,
    height: usize,
    stride: usize,
}

impl<'a, T> FrameAccessor<'a, T> {
    pub unsafe fn new_unchecked(data: &'a mut [T], width: usize, height: usize) -> Self {
        // SAFETY: width * height == data.len()
        Self {
            data,
            width,
            height,
            stride: width,
        }
    }

    pub fn new(data: &'a mut [T], width: usize, height: usize) -> Self {
        if data.len() != width * height {
            panic!("invalid size of slice passed to fixed frame")
        }
        // SAFETY: check just above
        unsafe { FrameAccessor::new_unchecked(data, width, height) }
    }
}

// SAFETY: width & height return fields that are never changed
unsafe impl<'a, T> FrameAccessorTrait for FrameAccessor<'a, T> {
    type Element = T;

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element {
        self.data.get_unchecked(x + y * self.stride)
    }

    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Element {
        self.data.get_unchecked_mut(x + y * self.stride)
    }
    fn data(&self) -> &[Self::Element] {
        self.data
    }
    fn data_mut(&mut self) -> &mut [Self::Element] {
        self.data
    }
}

impl<'a, T> Index<(usize, usize)> for FrameAccessor<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        FrameAccessorTrait::index(self, index)
    }
}

impl<'a, T> IndexMut<(usize, usize)> for FrameAccessor<'a, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        FrameAccessorTrait::index_mut(self, index)
    }
}

/// A frame accessor that has a size known at compile time
pub struct FixedFrameAccessor<'a, T, const W: usize, const H: usize> {
    data: &'a mut [T],
    stride: usize, // TODO: this can probably be made smaller, but do we care enough to increase complexity?
}

impl<'a, T, const W: usize, const H: usize> FixedFrameAccessor<'a, T, W, H> {
    pub unsafe fn new_unchecked(data: &'a mut [T]) -> Self {
        // SAFETY: L == W * H (can't use const generic params in const expressions, so no way to check this shit at compile-time)
        Self { data, stride: W }
    }

    pub fn new(data: &'a mut [T]) -> Self {
        if data.len() != W * H {
            panic!("invalid size of slice passed to fixed frame")
        }
        unsafe { FixedFrameAccessor::new_unchecked(data) }
    }
}

// SAFETY: width & height return constant generic parameters
unsafe impl<'a, T, const W: usize, const H: usize> FrameAccessorTrait
    for FixedFrameAccessor<'a, T, W, H>
{
    type Element = T;

    fn size(&self) -> (usize, usize) {
        (W, H)
    }

    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element {
        self.data.get_unchecked(x + y * self.stride)
    }

    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Element {
        self.data.get_unchecked_mut(x + y * self.stride)
    }

    fn data(&self) -> &[Self::Element] {
        self.data
    }
    fn data_mut(&mut self) -> &mut [Self::Element] {
        self.data
    }
}

impl<'a, T, const W: usize, const H: usize> Index<(usize, usize)>
    for FixedFrameAccessor<'a, T, W, H>
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        FrameAccessorTrait::index(self, index)
    }
}

impl<'a, T, const W: usize, const H: usize> IndexMut<(usize, usize)>
    for FixedFrameAccessor<'a, T, W, H>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        FrameAccessorTrait::index_mut(self, index)
    }
}

// not really useful, as it moves out of accessor (no way to get the fixed accessor back)
impl<'a, 'b, T, const W: usize, const H: usize> From<&'b mut FixedFrameAccessor<'a, T, W, H>>
    for FrameAccessor<'b, T>
{
    fn from(f: &'b mut FixedFrameAccessor<'a, T, W, H>) -> Self {
        // SAFETY: bounds should have been already checked for FixedFrameAccessor
        unsafe { Self::new_unchecked(f.data, W, H) }
    }
}
