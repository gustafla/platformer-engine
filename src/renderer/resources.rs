use std::vec:::Vec;
use gl::types::GLuint;

pub struct Resources {
    textures: Vec<GLuint>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            textures: Vec::with_capacity(1024),
        }
    }

    pub fn load_texture(mut &self, filename: &str) -> u32 {
    }
}
