extern crate sdl2;
extern crate gl;
extern crate image;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate render_gl_derive;

pub mod render_gl;
pub mod resources;
pub mod utils;

use render_gl::data::f32_f32_f32;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: f32_f32_f32,
    #[location = "1"]
    clr: f32_f32_f32,
}

fn main() {
    if let Err(e) = run() {
        println!("{}", utils::errors::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    use failure::err_msg;

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(err_msg)?;

    let gl = gl::Gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    use std::path::Path;
    use resources::Resources;
    use render_gl::Program;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;
    let shader = Program::from_res(&gl, &res, "shaders/triangle")?;

    shader.bind();

    let vertices: [Vertex; 3] = [
        Vertex {
            pos: (0.5,-0.5, 0.0).into(),
            clr: (1.0, 0.0, 0.0).into()
        },
        Vertex {
            pos: (-0.5,-0.5, 0.0).into(),
            clr: ( 0.0, 1.0, 0.0).into()
        },
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            clr: (0.0, 0.0, 1.0).into()
        }
    ];

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
    }

    Vertex::vertex_attrib_pointers(&gl);

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} => break 'main,
                _ => {},
            }
        }
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.BindVertexArray(vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();
    }

    Ok(())
}
