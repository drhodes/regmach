use crate::types::*;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
// use std::time::Duration;

impl LinuxDisplay {
    pub fn new() -> LinuxDisplay {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("register-machines", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        LinuxDisplay {
            ctx: sdl_context,
            canvas: canvas,
            zoom: 1,
        }
    }
}

impl Display for LinuxDisplay {
    fn exec(self: &mut Self, cmd: &Command) {
        match cmd {
            Command::FillScreen => self.canvas.clear(),
            Command::SetDrawColor(c) => {
                self.canvas.set_draw_color(Color::RGB(c.r, c.g, c.b));
            }
            Command::AddSegment(seg) => {
                let p1 = (seg.p1.x, seg.p1.y);
                let p2 = (seg.p2.x, seg.p2.y);
                self.canvas.draw_line(p1, p2);
            }
            _ => {}
        }
    }

    fn exec_cmds(self: &mut Self, cmds: Vec<Command>) {
        for c in cmds.iter() {
            self.exec(c)
        }
    }
}
