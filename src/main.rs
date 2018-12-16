extern crate gl;
extern crate sdl2;
extern crate image;
extern crate nalgebra;
extern crate vec_2_10_10_10;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

pub mod render_gl;
pub mod resources;
mod triangle;
mod debug;
mod input;

use nalgebra as na;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
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
    let mut viewport = render_gl::Viewport::for_window(900, 700);
    let color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    viewport.enable(&gl);
    color_buffer.enable(&gl);

    use std::path::Path;
    use resources::Resources;

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;
    let mut input = input::Input::new();
    let mut triangle = triangle::Triangle::new(&res, &gl)?;
    let mut translation = na::Vector3::new(0.0, 0.0, 0.0);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::{Event, WindowEvent};
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit {..} => break 'main,
                Event::Window {win_event: WindowEvent::Resized(w, h), ..} => {
                    viewport.update_size(w, h);
                    viewport.enable(&gl);
                },
                Event::KeyDown {keycode: Some(key), ..} |
                Event::KeyUp {keycode: Some(key), ..} => {
                    match key {
                        Keycode::Escape => break 'main,
                        _ => input.callback(&event),
                    }
                },
                _ => {},
            }
        }
        translation.x = (input.right - input.left) as f32 * 0.02;
        translation.y = (input.up - input.down) as f32 * 0.02;

        color_buffer.clear(&gl);
        &triangle.update_pos(&translation);
        triangle.render(&gl);
        window.gl_swap_window();
    }

    Ok(())
}
