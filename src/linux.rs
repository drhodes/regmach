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
        let cmds = vec![
            Command::IncrementFrame,
            Command::SetDrawColor(colors::BACKGROUND),
            Command::FillScreen,
            Command::SetDrawColor(colors::LIGHT_BLUE),
            Command::AddSegment(Segment::from_coords(10, 0, 10, 1000)),
            Command::SetDrawColor(colors::GREY),
        ];
        let mut last_cmds = vec![
            Command::SetDrawColor(if display.props.frame % 120 < 60 {
                colors::CURSOR_LIGHT
            } else {
                colors::CURSOR_DARK
            }),
            Command::RenderCursor,
        ];

        for event in display.get_events() {
            match event {
                Event::Quit => {
                    break 'running;
                }
                Event::MouseMove(pt) => {
                    display.props.mouse_loc = pt;
                }
                _ => {}
            }
        }

        display.exec_cmds(cmds);
        display.exec_cmds(schem.render());
        display.exec_cmds(last_cmds);
        display.canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
