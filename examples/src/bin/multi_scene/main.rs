#[path = "../../examples_common.rs"]
mod examples_common;

mod scene1;
mod scene2;

use std::io::{Write, self};

use scene1::Scene1;
use scene2::Scene2;

use examples_common::*;

use smol_tui::{enum_dispatch, FixedFrameAccessor};
use termion::{input::TermRead, event::Key, raw::IntoRawMode};

enum Event {
    Nothing,
    Button1,
    Button2,
    Button3,
    Button4,
}

#[enum_dispatch(Scenes)]
trait MyScene {
    fn render(&self, frame: &mut FixedFrameAccessor<char, 20, 4>, tick: u32);
    fn update(self, event: Event) -> Scenes;
}

#[enum_dispatch]
enum Scenes {
    Scene1,
    Scene2,
}


fn main() {
    let mut scene = Scenes::Scene1(Scene1::default());

    let mut frame = [' '; 20 * 4];

    let mut accessor = FixedFrameAccessor::new(&mut frame);


    println!(
        "Fun fact: Scenes is {} bytes long",
        std::mem::size_of::<Scenes>()
    );

    with_alternate_screen(|s| {

        let _stdout = io::stdout().into_raw_mode().unwrap();
        let stdin = termion::async_stdin();
        let mut keys = stdin.keys();

        let mut i = 0;
        loop {
            scene.render(&mut accessor, i);
            i += 1;

            // pre-format it
            let output = format!("{}\n{}", termion::clear::All, display_frame(&accessor))
                .replace("\n", "\r\n");

            // print in one go
            writeln!(s, "{}", output).unwrap();

            let mut scene_event = Event::Nothing;

            while let Some(event) = keys.next() {
                let event = event.unwrap();
                match event {
                    Key::Char('q') | Key::Ctrl('c') => return,
                    Key::Char('1') => scene_event = Event::Button1,
                    Key::Char('2') => scene_event = Event::Button2,
                    Key::Char('3') => scene_event = Event::Button3,
                    Key::Char('4') => scene_event = Event::Button4,
                    _ => {},
                }
            }

            // TODO: handle kb input
            scene = scene.update(scene_event);

            // (kinda) 60 Hz
            std::thread::sleep(std::time::Duration::from_micros(1000000 / 60))
        }
    });

    //println!("{}", display_frame(&accessor));
}
