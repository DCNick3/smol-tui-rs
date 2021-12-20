
use crate::{Scenes, MyScene, Event, Scene1};

use smol_tui::{smol_tui_scene, widgets, FixedFrameAccessor, FixedWidget};

#[smol_tui_scene(w = 20, h = 4, char_type = "char")]
pub struct Scene2 {
    #[smol_tui(x = 0, y = 0, w = 20, h = 4)]
    bg: widgets::Filler<char>,

    #[smol_tui(x = 9, y = 1, w = 2, h = 2)]
    square: widgets::Filler<char>,

    #[smol_tui(x = 0, y = 0, w = 1, h = 1)]
    spinner: widgets::Spinner<char>,
}

impl MyScene for Scene2 {
    fn render(&self, frame: &mut FixedFrameAccessor<char, 20, 4>, tick: u32) {
        self.bg.render_fixed(&' ', frame, tick);
        self.square.render_fixed(&'2', frame, tick);
        self.spinner.render_fixed(&(), frame, tick);
    }

    fn update(self, event: Event) -> Scenes {
        match event {
            Event::Button1 => Scenes::Scene1(Scene1::default()),
            _ => Scenes::Scene2(self),
        }
    }
}