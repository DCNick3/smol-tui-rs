#[path = "../examples_common.rs"]
mod examples_common;

use examples_common::*;

use std::io::Write;
use smol_tui::widgets::{Filler, Spinner};
use smol_tui::{widgets, Scene, FixedFrameAccessor};

// Note: chars are not that effective in terms of memory
// Used here just for the sake of spinner. Probably it should be moved to the examples...
#[derive(Scene)]
#[smol_tui(w = 20, h = 4, char_type = "char")]
struct TestScene {
    #[smol_tui(x = 0, y = 0, w = 20, h = 4)]
    bg: widgets::Filler<char>,

    #[smol_tui(skip)]
    #[allow(dead_code)]
    data_field: u32,

    #[smol_tui(x = 9, y = 1, w = 2, h = 2)]
    square: widgets::Filler<char>,

    #[smol_tui(x = 0, y = 0, w = 1, h = 1)]
    spinner: widgets::Spinner<char>,
}

fn main() {
    let scene = TestScene {
        bg: Filler::<char>::new(' '),
        square: Filler::<char>::new('X'),
        data_field: 2,
        spinner: Spinner::<char>::new(),
    };

    let mut frame = [' '; 20 * 4];

    let mut accessor = FixedFrameAccessor::new(&mut frame);

    with_alternate_screen(|s| {
        for i in 0..200 {
            scene.render(&mut accessor, i);
            writeln!(s, "{}{}", termion::clear::All, display_frame(&accessor)).unwrap();

            // (kinda) 60 Hz
            std::thread::sleep(std::time::Duration::from_micros(1000000 / 60))
        }
    });

    println!("Bye!");
}
