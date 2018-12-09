extern crate sdl2;
extern crate gl;
extern crate image;
extern crate vec_2_10_10_10;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

pub mod render_gl;
pub mod resources;
pub mod utils;

use render_gl::data::{f32_f32_f32, u2_u10_u10_u10_rev_float};

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: f32_f32_f32,
    #[location = "1"]
    clr: u2_u10_u10_u10_rev_float,
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
    use render_gl::{buffer::{VertexArray, VertexBuffer}, Program};

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;
    let shader = Program::from_res(&gl, &res, "shaders/triangle")?;

    shader.bind();

    let vertices: [Vertex; 3] = [
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            clr: (1.0, 0.0, 0.0, 1.0).into()
        }, // bottom right
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            clr: (0.0, 1.0, 0.0, 1.0).into()
        }, // bottom left
        Vertex {
            pos: (0.0,  0.5, 0.0).into(),
            clr: (0.0, 0.0, 1.0, 1.0).into()
        }  // top
    ];

    let vao= VertexArray::new(&gl);
    vao.bind();

    let vbo = VertexBuffer::new(&gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);

    Vertex::vertex_attrib_pointers(&gl);

    vao.unbind();
    vbo.unbind();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} => break 'main,
                _ => {},
            }
        }
        vao.bind();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();
    }

    Ok(())
}
