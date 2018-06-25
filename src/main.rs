extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod renderer;
use renderer::Renderer;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(2, 0);
    gl_attr.set_multisample_buffers(0);
    gl_attr.set_multisample_samples(0);

    let window = video.window("Plaformer game engine", 640, 480)
        .position_centered().opengl().build().unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(1., 0., 0., 1.);
    }

    let renderer = Renderer::new();

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
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        renderer.render();

        window.gl_swap_window();
    }
}
