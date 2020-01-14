use regmach::dsp::types as rdt;

pub struct LinuxDisplayOpenGl {
    pub ctx: glfw::Glfw,
    // pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    // pub event_pump: sdl2::EventPump,
    pub props: rdt::DisplayProperties,
}
