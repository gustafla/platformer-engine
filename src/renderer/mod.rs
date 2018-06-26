mod sprite;
mod mesh;

use std::vec::Vec;
use gl::{ActiveTexture, TEXTURE0};
use self::sprite::Sprite;
use self::mesh::Mesh;

pub struct Renderer {
    sprites: Vec<Sprite>,
    meshes: Vec<Mesh>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            sprites: vec![],
            meshes: vec![],
        }
    }

    pub fn render(&self) {
        unsafe {
            ActiveTexture(TEXTURE0);
        }
    }
}
