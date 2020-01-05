use crate::dsp::colors;
use crate::dsp::types::*;
use crate::schem::types::*;
use std::time::Duration;

pub fn start() {
    let _font_data = std::include_bytes!("../media/font/FontAwesome.otf");
    let mut display = LinuxDisplay::new();
    let mut schem = Schematic::new();

    schem.add_wire();

    'running: loop {
        for event in display.get_events() {
            match event {
                Event::Quit => {
                    break 'running;
                }
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
