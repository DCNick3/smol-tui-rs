#[path = "../examples_common.rs"]
mod examples_common;

use examples_common::*;

use smol_tui::{smol_tui_scene, widgets, FixedFrameAccessor, FixedWidget, Scene};
use std::io::Write;

// Note: chars are not that effective in terms of memory
// Used here just for the sake of spinner. Probably it should be moved to the examples...

#[smol_tui_scene(w = 20, h = 4, char_type = "char")]
struct TestScene {
    #[smol_tui(x = 0, y = 0, w = 20, h = 4)]
    bg: widgets::Filler<char>,

    #[smol_tui(skip, init = "42")] // init accepts a string that is later parsed as expression. May be suboptimal...
    #[allow(dead_code)]
    data_field: u32,

    #[smol_tui(x = 9, y = 1, w = 2, h = 2)]
    square: widgets::Filler<char>,

    #[smol_tui(x = 0, y = 0, w = 1, h = 1)]
    spinner: widgets::Spinner<char>,
}

impl Scene<char, 20, 4> for TestScene {
    fn render(&self, frame: &mut FixedFrameAccessor<char, 20, 4>, tick: u32) {
        self.bg.render_fixed(&' ', frame, tick);
        self.square.render_fixed(&'X', frame, tick);
        self.spinner.render_fixed(&(), frame, tick);
    }
}

fn main() {
    let scene: TestScene = Default::default();

    let mut frame = [' '; 20 * 4];

    let mut accessor = FixedFrameAccessor::new(&mut frame);

    println!(
        "Fun fact: TestScene is {} bytes long",
        std::mem::size_of::<TestScene>()
    );

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
