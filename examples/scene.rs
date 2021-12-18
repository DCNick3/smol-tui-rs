mod examples_common;

use examples_common::*;

use smol_tui::widgets::Filler;
use smol_tui::{widgets, Scene};

#[derive(Scene)]
#[smol_tui(w = 20, h = 4, char_type = "u8")]
struct TestScene {
    #[smol_tui(x = 0, y = 0, w = 20, h = 4)]
    bg: widgets::Filler<u8>,

    #[smol_tui(skip)]
    #[allow(dead_code)]
    data_field: u32,

    #[smol_tui(x = 9, y = 1, w = 2, h = 2)]
    square: widgets::Filler<u8>,
}

fn main() {
    let scene = TestScene {
        bg: Filler::<u8>::new(' ' as u8),
        square: Filler::<u8>::new('X' as u8),
        data_field: 2,
    };

    let mut frame: Frame = [' ' as u8; 20 * 4];

    let mut accessor = FixedAccessor::new(&mut frame);

    scene.render(&mut accessor);

    println!("{}", display_frame(&accessor));
}
