extern crate sdl2;
extern crate gl;

mod renderer;

use std::os::raw::c_void;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use renderer::Renderer;

fn main() {
    let size = (640, 480);
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(2, 0);
    gl_attr.set_multisample_buffers(0);
    gl_attr.set_multisample_samples(0);

    let window = video.window("Plaformer game engine", size.0 as u32, size.1 as u32)
        .position_centered().opengl().build().unwrap();
    let gl_context = window.gl_create_context().unwrap();

    // load OpenGL functions (necessary on Winblows)
    gl::load_with(|s| video.gl_get_proc_address(s) as *const c_void);

    unsafe {
        gl::Viewport(0, 0, size.0, size.1);
    }

    let mut renderer = Renderer::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop
                },
                _ => {}
            }
        }
        renderer.render();
        window.gl_swap_window();
    }
}
