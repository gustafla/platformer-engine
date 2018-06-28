mod sprite;
mod mesh;

use std::vec::Vec;
use gl::{ActiveTexture, BindTexture, TEXTURE0, TEXTURE_2D};
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
        for sprite in &self.sprites {
            unsafe {
                BindTexture(TEXTURE_2D, sprite.texture);
            }
        }
    }
}
