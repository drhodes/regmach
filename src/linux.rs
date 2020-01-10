use crate::dsp::colors;
use crate::dsp::fragment_shaders::*;
use crate::dsp::types::*;
use crate::dsp::vertex_shaders::*;
use crate::schem::types::*;

use gl;
use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::time::Duration;

// use lyon::math::{point, Point};
// use lyon::path::builder::*;
// use lyon::path::Path;
// use lyon::tessellation::basic_shapes::fill_circle;
// use lyon::tessellation::*;

//#[derive(Copy, Clone, Debug)]
// type MyVertex = [f32; 2];

// fn build_path() -> VertexBuffers<MyVertex, u16> {
// Build a Path.
// let mut builder = Path::builder();
// builder.move_to(point(0.0, 0.0));
// builder.line_to(point(1.0, 0.0));
// builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
// builder.cubic_bezier_to(point(1.0, 1.0), point(0.0, 1.0), point(0.0, 0.0));
// builder.close();
// let path = builder.build();

//     let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
//     let mut tessellator = FillTessellator::new();
//     {
//         // Compute the tessellation.
//         tessellator
//             .tessellate_path(
//                 &path,
//                 &FillOptions::default(),
//                 &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: FillAttributes| {
//                     pos.to_array()
//                 }),
//             )
//             .unwrap();
//     }
//     // The tessellated geometry is ready to be uploaded to the GPU.
//     println!(
//         " -- {} vertices {} indices",
//         geometry.vertices.len(),
//         geometry.indices.len()
//     );
//     geometry
// }

pub fn start() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader_program, vao, vbo) = gl_setup();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            let (r, g, b, a) = colors::BACKGROUND.as_gl();
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);

            let err = gl::GetError();
            if err != 0 {
                println!("opengl err: {:?}", err);
            }

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
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
            println!("{}\n {}", msg, str::from_utf8(&info_log).unwrap());
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
        "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
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
        "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
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
            "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
            str::from_utf8(&info_log).unwrap()
        );
    }
    shader_program
}

fn gl_setup() -> (u32, u32, u32) {
    // Thanks Matt!
    // https://gist.github.com/matthewjberger/9da00592b472b50ec1e6b3238719264b

    let (shader_program, vao, vbo) = unsafe {
        // -------------------------------------------------------
        // Setup shader compilation checks

        let mut success = i32::from(gl::FALSE);
        let mut info_log: Vec<u8> = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // -1 to skip trialing null character

        // Compile shaders
        let vertex_shader = make_vertex_shader(VERTEX_SHADER_SOURCE, &mut info_log);
        let fragment_shader = make_fragment_shader(FRAGMENT_SHADER_SOURCE, &mut info_log);

        // Link Shaders
        let shader_program = link_shaders(vertex_shader, fragment_shader, &mut info_log);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // Set up vao and vbos
        let vertices: [f32; 18] = [
            // left
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // top
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
        ];

        // stack allocate buffer ids
        let (mut vbo, mut vao) = (0, 0);
        // setup vertex array object and store id in vao
        gl::GenVertexArrays(1, &mut vao);
        // setup vertex buffer object and store id in vbo
        gl::GenBuffers(1, &mut vbo);
        // make vao current vertex array
        gl::BindVertexArray(vao);
        // make vbo current array buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

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

        // Enable a generic vertex attribute array
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Wireframe
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao, vbo)
    };
    (shader_program, vao, vbo)
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
