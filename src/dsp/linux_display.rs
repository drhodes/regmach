use crate::dsp::types::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

impl LinuxDisplay {
    pub fn new() -> LinuxDisplay {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("register-machines", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        LinuxDisplay {
            current_color: Color::RGB(0, 0, 0),
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
                self.current_color = Color::RGB(c.r, c.g, c.b);
                self.canvas.set_draw_color(Color::RGB(c.r, c.g, c.b));
            }
            Command::AddSegment(seg) => {
                // width of all lines? think about this. depends on
                // zoom, if zoom is a display property, or schematic
                // property.
                let line_width = 1;

                let p1 = (seg.p1.x, seg.p1.y);
                let p2 = (seg.p2.x, seg.p2.y);
                self.canvas.thick_line(
                    seg.p1.x as i16,
                    seg.p1.y as i16,
                    seg.p2.x as i16,
                    seg.p2.y as i16,
                    line_width,
                    self.current_color,
                );

                self.canvas.string(20, 20, "woot", self.current_color);
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
