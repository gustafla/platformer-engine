use std::vec::Vec;
use std::fs::File;
use std::io::BufReader;
use flif::Flif;
use gl::types::GLuint;

pub struct Resources {
    textures: Vec<GLuint>,
}

fn is_pot(size: (f32, f32)) -> bool {
    size.0.log2().ceil() == size.0.log2().floor()
        && size.1.log2().ceil() == size.1.log2().floor()
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            textures: Vec::with_capacity(1024),
        }
    }

    pub fn load_texture(&mut self, filename: &str) -> u32 {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let image = Flif::decode(reader).unwrap();
        if !is_pot((image.info().header.width as f32, image.info().header.height as f32)) {
            panic!("Texture {} size is not a power of two.", filename);
        }
        0
    }
}
