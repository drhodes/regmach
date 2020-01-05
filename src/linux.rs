extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::dsp::colors;
use crate::dsp::types::*;
use crate::schem::types::*;

pub fn start() {
    let mut display = LinuxDisplay::new();
    let mut schem = Schematic::new();

    schem.add_wire();

    let mut event_pump = display.ctx.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        // i += 1;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        display.exec_cmds(vec![
            Command::SetDrawColor(&colors::BACKGROUND),
            Command::FillScreen,
            Command::SetDrawColor(&colors::LIGHT_BLUE),
            Command::AddSegment(Segment::from_coords(10, 0, 10, 1000)),
            Command::SetDrawColor(&colors::GREY),
        ]);

        display.exec_cmds(schem.render());
        display.canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
