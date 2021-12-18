use core::fmt;
use std::{
    fmt::Write,
    io::{stdout, Stdout},
};

use smol_tui::{FixedFrameAccessor, FrameAccessor, FrameAccessorTrait};
use termion::screen::AlternateScreen;

#[allow(unused)]
pub fn with_alternate_screen<F>(f: F)
where
    F: FnOnce(&mut AlternateScreen<Stdout>) -> (),
{
    let mut screen = AlternateScreen::from(stdout());

    f(&mut screen);
}

pub struct DisplayableFrameAccessor<'a, T, F>(&'a F)
where
    F: FrameAccessorTrait<Element = T>,
    T: Into<char> + Copy; // can we get rid of Copy? do we want to?

impl<'a, T, F> fmt::Display for DisplayableFrameAccessor<'a, T, F>
where
    F: FrameAccessorTrait<Element = T>,
    T: Into<char> + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_border_row(f: &mut fmt::Formatter<'_>, width: usize) -> Result<(), fmt::Error> {
            f.write_char('+')?;
            for _ in 0..width {
                f.write_char('-')?;
            }
            f.write_char('+')?;
            Ok(())
        }

        let frame = &self.0;

        write_border_row(f, frame.width())?;
        f.write_char('\n')?;

        for y in 0..frame.height() {
            f.write_char('|')?;

            for x in 0..frame.width() {
                // SAFETY: x & y are in range by construction
                let c = *unsafe { frame.get_unchecked(x, y) };
                f.write_char(c.into())?
            }

            f.write_char('|')?;
            f.write_char('\n')?;
        }

        write_border_row(f, frame.width())?;

        Ok(())
    }
}

pub fn display_frame<T, F>(frame: &F) -> DisplayableFrameAccessor<T, F>
where
    F: FrameAccessorTrait<Element = T>,
    T: Into<char> + Copy,
{
    DisplayableFrameAccessor { 0: frame }
}

#[allow(unused)]
pub type FixedAccessor<'a> = FixedFrameAccessor<'a, u8, 20, 4>;
#[allow(unused)]
pub type Accessor<'a> = FrameAccessor<'a, u8>;
#[allow(unused)]
pub type Frame = [u8; 20 * 4];
