use crate::dsp::types::*;
use sdl2::event::Event as SdlEvent;
use sdl2::event::Event::MouseMotion as SdlMouseMotion;
use sdl2::gfx::primitives::DrawRenderer;
use std::path::Path;

use lyon::math::{point, Point};
use lyon::path::builder::*;
use lyon::path::Path as LPath;
use lyon::tessellation::*;

impl LinuxDisplay {
    pub fn new() -> LinuxDisplay {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        let window = video_subsystem
            .window("register-machines", 800, 800)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        sdl_context.mouse().show_cursor(false);
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        LinuxDisplay {
            ctx: sdl_context,
            canvas: canvas,
            event_pump: event_pump,
            props: DisplayProperties::new(),
        }
    }
}

impl Display for LinuxDisplay {
    fn exec(self: &mut Self, cmd: &Command) {
        match cmd {
            Command::FillScreen => {
                self.canvas.clear();
                // unsafe {
                //     gl::ClearColor(0.99, 0.99, 0.99, 1.0);
                //     gl::Clear(gl::COLOR_BUFFER_BIT);
                // }
            }

            Command::IncrementFrame => {
                self.props.frame += 1;
            }

            Command::SetDrawColor(c) => {
                self.props.current_color.r = c.r;
                self.props.current_color.g = c.g;
                self.props.current_color.b = c.b;
                self.canvas.set_draw_color(c.as_sdl());
            }

            Command::AddSegment(seg) => {
                // width of all lines? think about this. depends on
                // zoom, if zoom is a display property, or schematic
                // property.

                // ------------------------------------------------------------------

                let line_width = 1;

                let p1 = (seg.p1.x, seg.p1.y);
                let p2 = (seg.p2.x, seg.p2.y);
                self.canvas.thick_line(
                    seg.p1.x as i16,
                    seg.p1.y as i16,
                    seg.p2.x as i16,
                    seg.p2.y as i16,
                    line_width,
                    self.props.current_color.as_sdl(),
                );

                let frame_string = format!("{:?}", self.props.frame);
                self.canvas.string(
                    20,
                    20,
                    frame_string.as_str(),
                    self.props.current_color.as_sdl(),
                );
                self.canvas.draw_line(p1, p2);
            }

            Command::RenderCursor => {
                let p = &self.props.mouse_loc;

                // this function is incredibly slow when
                self.canvas.thick_line(
                    0 as i16,
                    p.y as i16,
                    10000 as i16,
                    p.y as i16,
                    1,
                    self.props.current_color.as_sdl(),
                );

                self.canvas.thick_line(
                    p.x as i16,
                    0 as i16,
                    p.x as i16,
                    10000 as i16,
                    1,
                    self.props.current_color.as_sdl(),
                );
            }
            _ => {}
        }
    }

    fn exec_cmds(self: &mut Self, cmds: Vec<Command>) {
        for c in cmds.iter() {
            self.exec(c)
        }
    }

    fn get_events(self: &mut Self) -> Vec<Event> {
        let mut evs: Vec<Event> = vec![];

        for event in self.event_pump.poll_iter() {
            match event {
                SdlEvent::Quit { .. } => {
                    evs.push(Event::Quit);
                }
                SdlMouseMotion {
                    x, y, mousestate, ..
                } => {
                    if mousestate.left() {
                        evs.push(Event::MouseDrag(DspPoint::new(x, y)))
                    } else {
                        evs.push(Event::MouseMove(DspPoint::new(x, y)))
                    }
                }
                x => {
                    println!("unhandled event: {:?}", x);
                }
            }
        }
        evs
    }
}
