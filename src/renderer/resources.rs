extern crate png;

use std::vec::Vec;
use std::fs::File;
use std::io::BufReader;
use gl::types::GLuint;

pub struct Resources {
    textures: Vec<GLuint>,
}

fn is_pot(width: f32, height: f32) -> bool {
    width.log2().ceil() == width.log2().floor()
        && height.log2().ceil() == height.log2().floor()
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            textures: Vec::with_capacity(1024),
        }
    }

    pub fn load_texture(&mut self, filename: &str) -> u32 {
        let file = File::open(filename).unwrap();
        let png_decoder = png::Decoder::new(file);
        let (png_info, mut png_reader) = png_decoder.read_info().unwrap();
        if !is_pot(png_info.width as f32, png_info.height as f32) {
            panic!("Texture {} size is not a power of two.", filename);
        }
        let mut image = vec![0; png_info.buffer_size()];
        png_reader.next_frame(&mut image).unwrap();
        0
    }
}
