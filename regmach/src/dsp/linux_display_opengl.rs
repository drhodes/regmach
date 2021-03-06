use crate::dsp;
use crate::dsp::fragment_shaders::*;
use crate::dsp::types::*;
use crate::dsp::vertex_shaders::*;
use crate::schem::types::*;
//use std::path::Path;

use gl;
use gl::types::*;
use glfw::{Action, Context, Key, MouseButton};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::time::Duration;

use lyon::math::{point, Point};
use lyon::path::builder::*;
use lyon::path::Path;
use lyon::tessellation::basic_shapes::fill_circle;
use lyon::tessellation::*;
use lyon_svg;

use rand::Rng;

impl LinuxDisplayOpenGl {
    pub fn new() -> LinuxDisplayOpenGl {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(
                600,
                600,
                "Register Machine Development Environment",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        window.make_current();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let geom = build_path();
        let (shader_program, vao, vbo) = gl_setup(&geom);

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                println!("event: {:?}", event);
                handle_window_event(&mut window, event);
            }

            unsafe {
                let (r, g, b, a) = dsp::colors::BACKGROUND.as_gl();
                //gl::FrontFace(gl::CW);
                gl::ClearColor(r, g, b, a);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::UseProgram(shader_program);
                gl::BindVertexArray(vao);

                let err = gl::GetError();
                if err != 0 {
                    println!("opengl err: {:?}", err);
                }
                gl::DrawElements(
                    gl::TRIANGLES,
                    geom.indices.len() as i32 * 3,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            }
            window.swap_buffers();
        }

        LinuxDisplayOpenGl {
            ctx: glfw,
            // canvas: canvas,
            // event_pump: event_pump,
            props: DisplayProperties::new(),
        }
    }
}

impl Display for LinuxDisplayOpenGl {
    fn exec(self: &mut Self, cmd: &Command) {
        match cmd {
            Command::FillScreen => {
                //self.canvas.clear();
                // unsafe {
                //     gl::ClearColor(0.99, 0.99, 0.99, 1.0);
                //     gl::Clear(gl::COLOR_BUFFER_BIT);
                // }
            }

            Command::IncrementFrame => {
                // self.props.frame += 1;
            }

            Command::SetDrawColor(c) => {
                // self.props.current_color.r = c.r;
                // self.props.current_color.g = c.g;
                // self.props.current_color.b = c.b;
                // self.canvas.set_draw_color(c.as_sdl());
            }

            Command::AddSegment(seg) => {
                // width of all lines? think about this. depends on
                // zoom, if zoom is a display property, or schematic
                // property.

                // ------------------------------------------------------------------

                // let line_width = 1;

                // let p1 = (seg.p1.x, seg.p1.y);
                // let p2 = (seg.p2.x, seg.p2.y);
                // self.canvas.thick_line(
                //     seg.p1.x as i16,
                //     seg.p1.y as i16,
                //     seg.p2.x as i16,
                //     seg.p2.y as i16,
                //     line_width,
                //     self.props.current_color.as_sdl(),
                // );

                // let frame_string = format!("{:?}", self.props.frame);
                // self.canvas.string(
                //     20,
                //     20,
                //     frame_string.as_str(),
                //     self.props.current_color.as_sdl(),
                // );
                // self.canvas.draw_line(p1, p2);
            }

            Command::RenderCursor => {
                // let p = &self.props.mouse_loc;

                // // this function is incredibly slow when
                // self.canvas.thick_line(
                //     0 as i16,
                //     p.y as i16,
                //     10000 as i16,
                //     p.y as i16,
                //     1,
                //     self.props.current_color.as_sdl(),
                // );

                // self.canvas.thick_line(
                //     p.x as i16,
                //     0 as i16,
                //     p.x as i16,
                //     10000 as i16,
                //     1,
                //     self.props.current_color.as_sdl(),
                // );
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

        // for event in self.event_pump.poll_iter() {
        //     match event {
        //         SdlEvent::Quit { .. } => {
        //             evs.push(Event::Quit);
        //         }
        //         SdlMouseMotion {
        //             x, y, mousestate, ..
        //         } => {
        //             if mousestate.left() {
        //                 evs.push(Event::MouseDrag(DspPoint::new(x, y)))
        //             } else {
        //                 evs.push(Event::MouseMove(DspPoint::new(x, y)))
        //             }
        //         }
        //         x => {
        //             println!("unhandled event: {:?}", x);
        //         }
        //     }
        // }
        evs
    }
}

type MyVertex = [f32; 2];
//const svg_string: &str = "M 0 0 L 1 0 L 1 1 L 0 1 z";
//const svg_string: &str = "M 0 0 H 1 V 1 H 1 L 1 1 z";
//const svg_string: &str = "M 0 0 C 20 20, 40 20, 50 10";
//const svg_string: &str = "M 0 0 L 0 2 L -1 0 L 0 1 Z";

fn build_path() -> VertexBuffers<MyVertex, u32> {
    let builder = Path::builder().with_svg();
    let path = lyon_svg::path_utils::build_path(builder, button).unwrap();

    // let mut builder = Path::builder();
    // builder.move_to(point(0.0, 0.0));
    // builder.line_to(point(0.0, 6.0));
    // builder.line_to(point(-2.0, 6.0));
    // builder.line_to(point(-2.0, 7.0));
    // builder.line_to(point(3.0, 7.0));
    // builder.line_to(point(3.0, 6.0));
    // builder.line_to(point(1.0, 6.0));
    // builder.line_to(point(1.0, 0.0));
    // builder.close();
    // let path = builder.build();

    let mut geometry: VertexBuffers<MyVertex, u32> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    {
        // Compute the tessellation.
        tessellator
            .tessellate_path(
                &path,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: FillAttributes| {
                    pos.to_array()
                }),
            )
            .unwrap();
    }
    // The tessellated geometry is ready to be uploaded to the GPU.
    println!(
        " -- {} vertices {} indexes",
        geometry.vertices.len(),
        geometry.indices.len()
    );
    geometry
}

fn gl_setup(geometry: &VertexBuffers<[f32; 2], u32>) -> (u32, u32, u32) {
    println!("geometry: {:?}", geometry);
    // -------------------------------------------------------
    // Setup shader compilation checks
    unsafe {
        let mut success = i32::from(gl::FALSE);
        let mut info_log: Vec<u8> = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // -1 to skip trialing null character

        // Compile shaders
        let vertex_shader = make_vertex_shader(VERTEX_SHADER_SOURCE, &mut info_log);
        let fragment_shader = make_fragment_shader(FRAGMENT_SHADER_SOURCE, &mut info_log);

        // Link Shaders
        let shader_program = link_shaders(vertex_shader, fragment_shader, &mut info_log);

        // [x,y,z]+
        // let vertices = [
        //     -1.0, -1.0, 0.0, //
        //     -0.5, -0.5, 0.0, //
        //     0.5, -0.5, 0.0 as f32, //
        //     0.0, 0.5, 0.0 as f32, //
        // ];

        // // [r,g,b]+
        // let colors = [
        //     1.0 as f32, 0.0, 0.0, //
        //     0.0, 1.0, 0.0, //
        //     0.0, 1.0, 1.0, //
        //     0.0, 0.0, 1.0, //
        // ];

        let mut vertices: Vec<f32> = vec![];
        for v in geometry.vertices.iter() {
            vertices.push(v[0] as f32);
            vertices.push(v[1] as f32);
            vertices.push(0.0);
        }

        let mut indexes = vec![];
        for i in geometry.indices.iter() {
            indexes.push((*i) as i32);
        }

        let mut rng = rand::thread_rng();
        let mut colors: Vec<f32> = vec![];
        for _ in 0..indexes.len() {
            // colors.push(rng.gen());
            // colors.push(rng.gen());
            // colors.push(rng.gen());
            let (r, g, b, _) = dsp::colors::JADE_BLUE.as_gl();
            colors.push(r);
            colors.push(g);
            colors.push(b);
        }

        // allocate object ids
        let mut vbo: u32 = 0;
        let mut vao: u32 = 0;
        let mut color_buffer_id: u32 = 0;
        let mut index_buffer_id: u32 = 0;

        // setup vertex array object and store id in vao
        gl::GenVertexArrays(1, &mut vao);
        // make vao current vertex array
        gl::BindVertexArray(vao);

        // VERTICES ------------------------------------------------------------------

        gl::GenBuffers(1, &mut vbo); // setup vertex buffer object and store id in vbo
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // make vbo current array buffer

        // creates and initializes a buffer object's data store
        gl::BufferData(
            // target
            gl::ARRAY_BUFFER,
            // size
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            // void *data
            &vertices[0] as *const f32 as *const c_void,
            // usage
            gl::STATIC_DRAW,
        );

        // define an array of generic vertex attribute data for verts
        gl::VertexAttribPointer(
            0, // shader location
            3, // with points made of 3 floats
            gl::FLOAT,
            gl::FALSE,   // not normalized
            0,           // stride
            ptr::null(), // offset
        );

        // Enable a generic vertex attribute array
        gl::EnableVertexAttribArray(0);

        // COLORS ------------------------------------------------------------------

        gl::GenBuffers(1, &mut color_buffer_id); // setup color buffer object
        gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer_id); // make color buffer current array buffer

        // creates and initializes a buffer object's data store
        gl::BufferData(
            // target
            gl::ARRAY_BUFFER,
            // size
            (colors.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            // void *data
            &colors[0] as *const f32 as *const c_void,
            // usage
            gl::STATIC_DRAW,
        );

        // define an array of generic vertex attribute data for verts
        gl::VertexAttribPointer(
            1, // shader location
            3, // with points made of 3 floats
            gl::FLOAT,
            gl::FALSE,   // not normalized
            0,           // stride
            ptr::null(), // offset
        );

        // Enable a generic vertex attribute array
        gl::EnableVertexAttribArray(1);

        // INDEXES ------------------------------------------------------------------
        gl::GenBuffers(2, &mut index_buffer_id);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_id);

        gl::BufferData(
            // target
            gl::ELEMENT_ARRAY_BUFFER,
            // size
            (indexes.len() * mem::size_of::<u32>()) as GLsizeiptr,
            // void *data
            &indexes[0] as *const i32 as *const c_void,
            // usage
            gl::STATIC_DRAW,
        );

        check_gl_error();
        // Wireframe
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        (shader_program, vao, vbo)
    }
}

unsafe fn check_gl_error() {
    let err = gl::GetError();
    if err != 0 {
        println!("opengl err: {:?}", err);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::FramebufferSize(x, y) => {
            println!("framebuffersize {:?}, {:?}", x, y);
        }
        _ => {}
    }
}

fn check_shader_err(vertex_shader: u32, msg: &str, info_log: &mut Vec<u8>) {
    unsafe {
        let mut success = i32::from(gl::FALSE);
        // Check for shader compilation errors
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!("{}{}", msg, str::from_utf8(&info_log).unwrap());
        }
    }
}

unsafe fn make_vertex_shader(src: &str, info_log: &mut Vec<u8>) -> u32 {
    // Vertex shadeal
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertex_shader);

    // Check for shader compilation errors
    check_shader_err(
        vertex_shader,
        "ERROR::SHADER::VERTEX::COMPILATION_FAILED{}",
        info_log,
    );
    vertex_shader
}

unsafe fn make_fragment_shader(src: &str, info_log: &mut Vec<u8>) -> u32 {
    // Fragment shader
    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_vert = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(fragment_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(fragment_shader);

    // Check for shader compilation errors
    check_shader_err(
        fragment_shader,
        "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED{}",
        info_log,
    );
    fragment_shader
}

unsafe fn link_shaders(vertex_shader: u32, fragment_shader: u32, info_log: &mut Vec<u8>) -> u32 {
    let mut success = i32::from(gl::FALSE);
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    // Check for linking errors
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != i32::from(gl::TRUE) {
        gl::GetProgramInfoLog(
            shader_program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::PROGRAM::COMPILATION_FAILED{}",
            str::from_utf8(&info_log).unwrap()
        );
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);
    shader_program
}

// pub fn start() {
//     let _font_data = std::include_bytes!("../media/font/FontAwesome.otf");
//     let mut display = LinuxDisplay::new();
//     let mut schem = Schematic::new();

//     schem.add_wire();

//     'running: loop {
//         let cmds = vec![
//             Command::IncrementFrame,
//             Command::SetDrawColor(colors::BACKGROUND),
//             Command::FillScreen,
//             Command::SetDrawColor(colors::LIGHT_BLUE),
//             Command::AddSegment(Segment::from_coords(10, 0, 10, 1000)),
//             Command::SetDrawColor(colors::GREY),
//         ];
//         let mut last_cmds = vec![
//             Command::SetDrawColor(if display.props.frame % 120 < 60 {
//                 colors::CURSOR_LIGHT
//             } else {
//                 colors::CURSOR_DARK
//             }),
//             Command::RenderCursor,
//         ];

//         for event in display.get_events() {
//             match event {
//                 Event::Quit => {
//                     break 'running;
//                 }
//                 Event::MouseMove(pt) => {
//                     display.props.mouse_loc = pt;
//                 }
//                 _ => {}
//             }
//         }

//         display.exec_cmds(cmds);
//         display.exec_cmds(schem.render());
//         display.exec_cmds(last_cmds);
//         display.canvas.present();

//         std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//     }
// }

// unsafe fn compile_shader(shader_type: u32, err: &str, src: &str, info_log: &mut Vec<u8>) -> u32 {
//     let shader = gl::CreateShader(shader_type);
//     let c_str_vert = CString::new(src.as_bytes()).unwrap();
//     gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
//     gl::CompileShader(shader);

//     // Check for shader compilation errors
//     check_shader_err(shader, "Compile Error: {:?}, {:?}", err, info_log);
//     shader
// }

const letter_s: &str = r#"m -0.771406,0.330125 c -0.268529,-0.31785 -0.668582,-0.493216 -1.101516,-0.493216 -0.712424,0 -1.20564,0.476776 -1.20564,1.013833 0,0.235648 0.09316,0.531578 0.421974,0.772706 0.367172,0.274009 0.937111,0.411013 1.238521,0.59734 0.257568,0.164405 0.328811,0.367172 0.328811,0.548018 0,0.339771 -0.29045,0.717903 -0.860389,0.717903 -0.438414,0 -0.772705,-0.213726 -0.948071,-0.438414 -0.02192,-0.0274 -0.01644,-0.04384 -0.01644,-0.06576 0,-0.01096 0,-0.04932 -0.0274,-0.06028 l -0.26305,0.454851 c 0.30689,0.334291 0.750785,0.509657 1.238521,0.509657 0.964512,0 1.320723,-0.646662 1.320723,-1.145358 0,-0.208247 -0.06028,-0.482256 -0.30689,-0.717904 -0.345251,-0.323331 -0.90423,-0.432934 -1.265922,-0.646661 -0.32881,-0.191807 -0.405533,-0.427454 -0.405533,-0.59186 0,-0.30141 0.263049,-0.580899 0.717904,-0.580899 0.367172,0 0.652141,0.169886 0.805586,0.394573 0.01644,0.02192 0.01096,0.03836 0.01096,0.06028 0.0055,0.03288 0.02192,0.04932 0.03836,0.0548 "#;

const letter_t: &str = "m 0 0 v 1 h 3 v 10 h 1 v -10 h 3 v -1 z";

const button: &str = "m -0.599275,0.63873 c -0.02686,0 -0.05144,9.6e-4 -0.07422,0.002 -0.02664,10e-4 -0.05173,0.002 -0.07422,0.004 -0.02638,0.002 -0.05008,0.007 -0.07227,0.01 h -0.002 -0.002 c -0.02335,0.004 -0.0465,0.008 -0.06836,0.0117 l -0.002,0.002 c -0.02607,0.005 -0.04876,0.011 -0.06836,0.0156 h -0.002 -0.002 c -0.0253,0.007 -0.04715,0.014 -0.06641,0.0195 h -0.002 c -0.02482,0.008 -0.04749,0.017 -0.06641,0.0234 h -0.002 c -0.02395,0.009 -0.04555,0.0178 -0.06445,0.0254 h -0.002 c -0.02344,0.01 -0.04561,0.0206 -0.06445,0.0293 l -0.002,0.002 c -0.02119,0.0102 -0.04153,0.0198 -0.0625,0.0313 -0.0213,0.0115 -0.04078,0.0232 -0.06055,0.0351 -0.02036,0.0123 -0.04107,0.0259 -0.06055,0.0391 -0.01945,0.0132 -0.03782,0.027 -0.05664,0.041 -0.01882,0.0141 -0.03851,0.028 -0.05664,0.043 -0.01788,0.0148 -0.03509,0.0308 -0.05273,0.0469 -0.0172,0.0157 -0.03413,0.0302 -0.05078,0.0469 -0.01664,0.0166 -0.03313,0.0335 -0.04883,0.0508 -0.01568,0.0172 -0.02986,0.0364 -0.04492,0.0547 -0.01489,0.0181 -0.03088,0.0359 -0.04492,0.0547 -0.01405,0.0188 -0.02771,0.0369 -0.04102,0.0566 -0.01334,0.0197 -0.02497,0.0406 -0.03711,0.0605 -0.01215,0.0201 -0.02383,0.0397 -0.03516,0.0606 -0.01119,0.0207 -0.02286,0.043 -0.0332,0.0644 -0.01026,0.0212 -0.0199,0.0421 -0.0293,0.0645 -0.0094,0.0222 -0.01724,0.0442 -0.02539,0.0664 -0.0083,0.0225 -0.01629,0.0454 -0.02344,0.0684 -0.0072,0.023 -0.01351,0.047 -0.01953,0.0703 -0.0061,0.0235 -0.0127,0.0462 -0.01758,0.0703 -0.0049,0.0239 -0.008,0.0481 -0.01172,0.0723 -0.0038,0.0245 -0.0073,0.0498 -0.0098,0.0742 -0.0025,0.0249 -0.0046,0.0496 -0.0059,0.0742 -0.0012,0.0249 -0.002,0.049 -0.002,0.0742 0,0.025 6.08e-4,0.0507 0.002,0.0762 0.0013,0.0248 0.0033,0.0494 0.0059,0.0742 0.0025,0.0244 0.006,0.0497 0.0098,0.0742 0.0037,0.0248 0.007,0.0489 0.01172,0.0723 0.0049,0.024 0.0115,0.0468 0.01758,0.0703 0.006,0.0234 0.01239,0.0473 0.01953,0.0703 0.0071,0.023 0.01512,0.0457 0.02344,0.0684 0.0083,0.0225 0.0163,0.0448 0.02539,0.0664 0.0092,0.0217 0.01893,0.0429 0.0293,0.0645 0.01026,0.0212 0.0219,0.0417 0.0332,0.0625 0.01121,0.0206 0.02276,0.0421 0.03516,0.0625 0.01216,0.0199 0.02381,0.039 0.03711,0.0586 0.01305,0.0193 0.02682,0.0396 0.04102,0.0586 0.01405,0.0188 0.03003,0.0366 0.04492,0.0547 0.01506,0.0183 0.02924,0.0355 0.04492,0.0527 0.01568,0.0172 0.03214,0.0341 0.04883,0.0508 0.01668,0.0167 0.03361,0.0332 0.05078,0.0488 0.0172,0.0156 0.0346,0.0319 0.05273,0.0469 0.01813,0.015 0.03782,0.0289 0.05664,0.043 0.01882,0.0141 0.03719,0.0279 0.05664,0.041 0.01945,0.0132 0.04015,0.0267 0.06055,0.0391 0.02009,0.0122 0.03991,0.0239 0.06055,0.0352 0.01971,0.0107 0.04053,0.0221 0.06445,0.0332 v -0.002 c 0.01953,0.009 0.04114,0.0211 0.06641,0.0312 0.01899,0.008 0.03995,0.0169 0.06445,0.0254 0.02022,0.007 0.04242,0.0159 0.06836,0.0234 0.02115,0.007 0.04504,0.0132 0.07227,0.0195 0.02176,0.006 0.044,0.0114 0.06641,0.0156 0.02454,0.005 0.05011,0.0101 0.07617,0.0137 0.02219,0.003 0.0459,0.005 0.07227,0.008 0.0219,0.002 0.04643,0.005 0.07227,0.006 0.02602,0.002 0.05091,0.002 0.07617,0.002 0.02687,0 0.05144,-9.5e-4 0.07422,-0.002 0.02664,-10e-4 0.05173,-0.004 0.07422,-0.006 0.02638,-0.002 0.05008,-0.005 0.07227,-0.008 h 0.002 0.002 c 0.02335,-0.004 0.0465,-0.01 0.06836,-0.0137 h 0.002 c 0.02609,-0.005 0.04876,-0.011 0.06836,-0.0156 h 0.002 0.002 c 0.02154,-0.006 0.04572,-0.0126 0.07227,-0.0215 0.02371,-0.007 0.04428,-0.0153 0.0625,-0.0215 h 0.002 c 0.02395,-0.009 0.04555,-0.0178 0.06445,-0.0254 h 0.002 v -0.002 c 0.02344,-0.01 0.04561,-0.0186 0.06445,-0.0273 v -0.002 h 0.002 c 0.02119,-0.0102 0.04153,-0.0198 0.0625,-0.0312 0.02132,-0.0115 0.04274,-0.0232 0.0625,-0.0352 0.02036,-0.0123 0.03912,-0.0259 0.05859,-0.0391 0.01945,-0.0131 0.03977,-0.0269 0.05859,-0.041 0.01882,-0.0141 0.03656,-0.028 0.05469,-0.043 0.01788,-0.0148 0.03509,-0.0308 0.05273,-0.0469 0.0172,-0.0157 0.03413,-0.0322 0.05078,-0.0488 0.01664,-0.0166 0.03313,-0.0335 0.04883,-0.0508 0.01568,-0.0173 0.03182,-0.0345 0.04687,-0.0527 0.01489,-0.0181 0.02892,-0.0359 0.04297,-0.0547 0.01405,-0.0187 0.02771,-0.0388 0.04102,-0.0586 0.01334,-0.0197 0.02497,-0.0387 0.03711,-0.0586 0.01238,-0.0203 0.02588,-0.0418 0.03711,-0.0625 0.01129,-0.0208 0.0209,-0.041 0.03125,-0.0625 0.01026,-0.0213 0.0199,-0.0421 0.0293,-0.0645 0.0094,-0.0222 0.0192,-0.0443 0.02734,-0.0664 0.0083,-0.0225 0.01629,-0.0454 0.02344,-0.0684 0.0072,-0.023 0.0135,-0.047 0.01953,-0.0703 0.0061,-0.0235 0.01074,-0.0462 0.01563,-0.0703 0.0049,-0.0239 0.01,-0.0481 0.01367,-0.0723 0.0038,-0.0245 0.0053,-0.0498 0.0078,-0.0742 0.0025,-0.0249 0.0046,-0.0496 0.0059,-0.0742 0.0012,-0.0249 0.002,-0.0509 0.002,-0.0762 0,-0.025 -6.08e-4,-0.0488 -0.002,-0.0742 -0.0013,-0.0248 -0.0033,-0.0494 -0.0059,-0.0742 -0.0025,-0.0244 -0.004,-0.0497 -0.0078,-0.0742 -0.0037,-0.0248 -0.0089,-0.0489 -0.01367,-0.0723 -0.0049,-0.024 -0.0096,-0.0468 -0.01563,-0.0703 -0.006,-0.0234 -0.01239,-0.0473 -0.01953,-0.0703 -0.0071,-0.023 -0.01512,-0.0457 -0.02344,-0.0684 -0.0083,-0.0225 -0.01825,-0.0448 -0.02734,-0.0664 -0.0092,-0.0217 -0.01893,-0.0429 -0.0293,-0.0645 -0.01033,-0.0215 -0.01994,-0.0436 -0.03125,-0.0644 -0.01121,-0.0206 -0.02471,-0.0402 -0.03711,-0.0606 -0.01216,-0.0199 -0.02382,-0.0409 -0.03711,-0.0605 -0.01305,-0.0193 -0.02682,-0.0377 -0.04102,-0.0566 -0.01405,-0.0188 -0.02808,-0.0366 -0.04297,-0.0547 -0.01506,-0.0183 -0.03119,-0.0374 -0.04687,-0.0547 -0.01568,-0.0172 -0.03214,-0.0341 -0.04883,-0.0508 -0.01668,-0.0167 -0.03361,-0.0312 -0.05078,-0.0469 v -0.002 c -0.0172,-0.0156 -0.0346,-0.03 -0.05273,-0.0449 -0.01813,-0.015 -0.03587,-0.0289 -0.05469,-0.043 -0.01882,-0.0141 -0.03914,-0.0279 -0.05859,-0.041 -0.01945,-0.0131 -0.03819,-0.0267 -0.05859,-0.0391 -0.02008,-0.0121 -0.04186,-0.0239 -0.0625,-0.0351 -0.01924,-0.0105 -0.03926,-0.0224 -0.0625,-0.0332 -0.01906,-0.009 -0.0419,-0.0194 -0.06641,-0.0293 -0.01948,-0.008 -0.04102,-0.0166 -0.06641,-0.0254 -0.02431,-0.009 -0.04507,-0.015 -0.06445,-0.0215 l -0.002,-0.002 c -0.02115,-0.007 -0.04699,-0.0131 -0.07422,-0.0195 h 0.0059 c -0.02352,-0.007 -0.04799,-0.011 -0.07227,-0.0156 -0.02454,-0.005 -0.05011,-0.0101 -0.07617,-0.0137 -0.02219,-0.003 -0.04588,-0.007 -0.07227,-0.01 -0.02131,-0.002 -0.04527,-0.003 -0.07031,-0.004 -0.02602,-0.002 -0.05287,-0.002 -0.07813,-0.002 z m 0,0.26368 c 0.02082,0 0.04107,7.3e-4 0.06055,0.002 h 0.002 c 0.01886,8.3e-4 0.04013,0.002 0.0625,0.004 0.0185,0.002 0.03855,0.005 0.06055,0.008 0.01812,0.002 0.03774,0.006 0.05664,0.01 h 0.002 l 0.002,0.002 c 0.01772,0.003 0.03494,0.007 0.05273,0.0117 h 0.002 l 0.0039,0.002 c 0.01541,0.004 0.03209,0.009 0.05273,0.0156 0.0213,0.007 0.04071,0.0117 0.05664,0.0176 h 0.002 v 0.002 c 0.01467,0.005 0.03279,0.0111 0.05273,0.0195 l 0.002,0.002 c 0.01466,0.006 0.03141,0.0142 0.05078,0.0234 h 0.002 c 0.01497,0.007 0.03267,0.0155 0.05078,0.0254 0.01718,0.009 0.03221,0.0192 0.04883,0.0293 0.0163,0.01 0.03278,0.0204 0.04883,0.0312 0.01605,0.0109 0.03339,0.0217 0.04883,0.0332 0.0087,0.006 0.01691,0.0128 0.02539,0.0195 l -0.01172,0.0117 -0.05664,0.0586 -0.05859,0.0586 -0.05859,0.0586 -0.05859,0.0566 -0.05859,0.0586 -0.05859,0.0586 -0.05859,0.0586 -0.05664,0.0586 -0.05859,0.0586 -0.05859,0.0586 -0.05859,0.0566 -0.05859,0.0586 -0.04687,0.0469 -0.0098,-0.01 -0.05859,-0.0605 -0.06055,-0.0586 -0.05859,-0.0586 -0.05859,-0.0606 -0.06055,-0.0586 -0.05859,-0.0586 -0.05859,-0.0606 -0.06055,-0.0586 -0.05859,-0.0586 -0.05859,-0.0605 -0.06055,-0.0586 -0.05859,-0.0586 -0.02539,-0.0254 c 0.01307,-0.0107 0.02552,-0.0211 0.03906,-0.0312 0.01544,-0.0115 0.03083,-0.0224 0.04687,-0.0332 0.01603,-0.0108 0.0325,-0.0214 0.04883,-0.0312 0.01694,-0.0102 0.03426,-0.0204 0.05078,-0.0293 0.01608,-0.009 0.03197,-0.0172 0.04883,-0.0254 l 0.002,-0.002 c 0.0182,-0.008 0.03557,-0.015 0.05078,-0.0215 l 0.002,-0.002 c 0.01858,-0.007 0.0374,-0.0138 0.05273,-0.0195 l 0.0039,-0.002 c 0.01946,-0.007 0.03732,-0.0127 0.05273,-0.0176 0.02255,-0.006 0.04123,-0.0131 0.05859,-0.0176 l -0.002,0.002 c 0.02304,-0.005 0.04318,-0.0121 0.06055,-0.0156 l -0.002,0.002 c 0.02027,-0.004 0.03898,-0.008 0.05859,-0.0117 0.022,-0.003 0.04205,-0.006 0.06055,-0.008 0.02237,-0.002 0.04364,-0.003 0.0625,-0.004 0.02273,-10e-4 0.0433,-0.002 0.0625,-0.002 z m 0.933594,0.44531 c 0.01015,0.0125 0.02162,0.0242 0.03125,0.0371 0.01142,0.0152 0.02228,0.0307 0.0332,0.0469 0.01069,0.0158 0.02111,0.0322 0.03125,0.0488 0.0099,0.0162 0.01994,0.0336 0.0293,0.0508 0.0092,0.017 0.01696,0.0352 0.02539,0.0527 0.0084,0.0175 0.01578,0.0347 0.02344,0.0527 0.0077,0.0184 0.01673,0.0364 0.02344,0.0547 0.0067,0.0182 0.01173,0.0358 0.01758,0.0547 0.0059,0.0188 0.01068,0.0374 0.01563,0.0566 v 0.002 c 0.0049,0.0191 0.0097,0.0371 0.01367,0.0566 0.0041,0.02 0.0088,0.0392 0.01172,0.0586 v 0.002 c 0.003,0.0197 0.0057,0.0382 0.0078,0.0586 0.0021,0.02 0.0028,0.0417 0.0039,0.0625 0.0011,0.0199 0.002,0.0394 0.002,0.0605 0,0.0208 -9.38e-4,0.0418 -0.002,0.0625 -0.0011,0.0207 -0.0018,0.0404 -0.0039,0.0605 -0.0021,0.0204 -0.0048,0.0409 -0.0078,0.0605 -0.0031,0.0201 -0.0077,0.0411 -0.01172,0.0606 -0.0039,0.0193 -0.0087,0.0375 -0.01367,0.0566 -0.005,0.0193 -0.0098,0.0378 -0.01563,0.0566 -0.0059,0.0188 -0.01283,0.0384 -0.01953,0.0566 h 0.002 c -0.0069,0.0187 -0.01402,0.0371 -0.02148,0.0547 h -0.002 c -0.0074,0.0176 -0.01496,0.0352 -0.02344,0.0527 -0.0085,0.0175 -0.01612,0.0357 -0.02539,0.0527 -0.0093,0.0171 -0.01938,0.0325 -0.0293,0.0488 -0.01016,0.0167 -0.02059,0.0331 -0.03125,0.0488 v 0.002 c -0.01068,0.0159 -0.02163,0.0314 -0.0332,0.0469 -0.0099,0.0133 -0.02077,0.0262 -0.03125,0.0391 l -0.0078,-0.008 -0.05859,-0.0606 -0.05859,-0.0586 -0.06055,-0.0586 -0.05859,-0.0606 -0.05859,-0.0586 -0.06055,-0.0586 -0.05859,-0.0605 -0.05859,-0.0586 -0.06055,-0.0586 -0.05859,-0.0605 -0.05859,-0.0586 -0.06055,-0.0586 -0.03906,-0.0391 0.04687,-0.0469 0.05859,-0.0586 0.05859,-0.0586 0.05664,-0.0586 0.05859,-0.0566 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 0.05664,-0.0566 0.05859,-0.0586 0.05859,-0.0586 z m -1.876953,0.0117 0.02344,0.0254 0.06055,0.0586 0.05859,0.0586 0.05859,0.0586 0.06055,0.0605 0.05859,0.0586 0.05859,0.0586 0.06055,0.0605 0.05859,0.0586 0.05859,0.0586 0.06055,0.0605 0.05859,0.0586 0.05859,0.0586 0.01172,0.0117 -0.05859,0.0566 -0.05859,0.0586 -0.05859,0.0586 -0.05859,0.0586 -0.05859,0.0586 -0.05664,0.0586 -0.05859,0.0586 -0.05859,0.0566 -0.05859,0.0586 -0.05859,0.0586 -0.05859,0.0586 -0.05859,0.0586 -0.04492,0.0449 c -0.0067,-0.009 -0.01303,-0.0167 -0.01953,-0.0254 -0.01142,-0.0153 -0.02228,-0.0327 -0.0332,-0.0488 -0.01069,-0.0158 -0.02111,-0.0322 -0.03125,-0.0488 -0.0099,-0.0162 -0.01994,-0.0316 -0.0293,-0.0488 -0.0092,-0.017 -0.01884,-0.0352 -0.02734,-0.0527 -0.0084,-0.0175 -0.01578,-0.0347 -0.02344,-0.0527 -0.0077,-0.0184 -0.01477,-0.0364 -0.02148,-0.0547 -0.0067,-0.0182 -0.01368,-0.0358 -0.01953,-0.0547 v -0.002 c -0.0059,-0.0188 -0.01068,-0.0374 -0.01563,-0.0566 -0.0049,-0.0191 -0.0097,-0.0371 -0.01367,-0.0566 -0.0041,-0.02 -0.0068,-0.0411 -0.0098,-0.0606 -0.003,-0.0197 -0.0057,-0.0401 -0.0078,-0.0605 -0.0021,-0.02 -0.0048,-0.0397 -0.0059,-0.0605 -0.0011,-0.0199 0,-0.0413 0,-0.0625 0,-0.0208 -10e-4,-0.0398 0,-0.0605 0.0011,-0.0207 0.0038,-0.0423 0.0059,-0.0625 0.0021,-0.0204 0.0048,-0.0389 0.0078,-0.0586 0.0031,-0.0201 0.0058,-0.0411 0.0098,-0.0605 0.0039,-0.0193 0.0087,-0.0375 0.01367,-0.0566 0.005,-0.0193 0.0098,-0.0398 0.01563,-0.0586 0.0059,-0.0188 0.01282,-0.0364 0.01953,-0.0547 0.0068,-0.0186 0.014,-0.037 0.02148,-0.0547 0.0074,-0.0176 0.01496,-0.0352 0.02344,-0.0527 0.0085,-0.0175 0.01798,-0.0354 0.02734,-0.0527 0.0092,-0.017 0.01915,-0.034 0.0293,-0.0508 0.01016,-0.0167 0.02059,-0.0331 0.03125,-0.0488 0.01068,-0.0159 0.02163,-0.0314 0.0332,-0.0469 0.0065,-0.009 0.0128,-0.0169 0.01953,-0.0254 z m 0.93164,0.93164 0.04102,0.041 0.05859,0.0586 0.05859,0.0586 0.05859,0.0605 0.06055,0.0586 0.05859,0.0586 0.05859,0.0605 0.06055,0.0586 0.05859,0.0586 0.05859,0.0606 0.06055,0.0586 0.05859,0.0586 0.05859,0.0586 0.0078,0.008 c -0.0083,0.007 -0.01691,0.0132 -0.02539,0.0195 -0.01544,0.0115 -0.03278,0.0224 -0.04883,0.0332 -0.01603,0.0108 -0.0325,0.0214 -0.04883,0.0312 -0.01694,0.0102 -0.03231,0.0184 -0.04883,0.0274 v 0.002 c -0.01685,0.009 -0.03502,0.0168 -0.05273,0.0254 -0.01935,0.009 -0.03675,0.0167 -0.05273,0.0234 -0.01942,0.008 -0.03685,0.0156 -0.05273,0.0215 -0.02197,0.008 -0.04162,0.0143 -0.05859,0.0195 l -0.002,0.002 c -0.0141,0.005 -0.03169,0.009 -0.05078,0.0137 -0.02304,0.005 -0.04513,0.0102 -0.0625,0.0137 -0.02027,0.004 -0.04094,0.008 -0.06055,0.0117 -0.02038,0.003 -0.03933,0.004 -0.05664,0.006 -0.02239,0.002 -0.04364,0.005 -0.0625,0.006 -0.02272,0.001 -0.04329,0.002 -0.0625,0.002 -0.02082,0 -0.04108,-7.4e-4 -0.06055,-0.002 h -0.002 c -0.01886,-8.3e-4 -0.04013,-0.004 -0.0625,-0.006 -0.01849,-0.002 -0.03856,-0.003 -0.06055,-0.006 -0.01812,-0.002 -0.03579,-0.006 -0.05469,-0.01 l -0.002,-0.002 h -0.002 c -0.01772,-0.003 -0.03692,-0.007 -0.05469,-0.0117 l -0.002,-0.002 h -0.002 c -0.01542,-0.004 -0.03404,-0.009 -0.05469,-0.0156 h -0.002 c -0.01503,-0.004 -0.03257,-0.0102 -0.05273,-0.0176 l -0.002,-0.002 h -0.002 c -0.01467,-0.005 -0.03279,-0.0131 -0.05273,-0.0215 h -0.002 c -0.01466,-0.006 -0.03141,-0.0142 -0.05078,-0.0234 h -0.002 c -0.01497,-0.007 -0.03072,-0.0155 -0.04883,-0.0254 -0.01718,-0.009 -0.03419,-0.0193 -0.05078,-0.0293 -0.0163,-0.01 -0.03278,-0.0204 -0.04883,-0.0312 -0.01605,-0.0108 -0.03144,-0.0217 -0.04687,-0.0332 -0.01325,-0.01 -0.02626,-0.0208 -0.03906,-0.0312 l 0.04687,-0.0449 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 0.05664,-0.0586 0.05859,-0.0566 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 0.05859,-0.0586 z";
