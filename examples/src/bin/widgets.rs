#[path = "../examples_common.rs"]
mod examples_common;

use examples_common::*;

use smol_tui::{widgets, FixedWidget, Widget};

fn main() {
    let mut frame: Frame = [' ' as u8; 20 * 4];

    let filler = widgets::Filler::<u8>::new();
    {
        let mut accessor = Accessor::new(&mut frame, 20, 4);
        let accessor = &mut accessor;

        filler.render(&('X' as u8), accessor, 0);
        accessor[(1, 2)] = 'y' as u8;

        println!("{}", display_frame(accessor));
    }

    {
        let mut accessor = FixedAccessor::new(&mut frame);
        let accessor = &mut accessor;

        let filler = widgets::Filler::<u8>::new();

        filler.render_fixed(&('Y' as u8), accessor, 0);

        accessor[(4, 3)] = 'x' as u8;

        println!("{}", display_frame(accessor));
    }
}
