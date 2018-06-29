extern crate gl;

mod sprite;
mod mesh;
mod resources;

use std::vec::Vec;
use self::sprite::Sprite;
use self::mesh::Mesh;
use self::resources::Resources;

pub struct Renderer {
    resources: Resources,
    sprites: Vec<Sprite>,
    meshes: Vec<Mesh>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            resources: Resources::new(),
            sprites: vec![],
            meshes: vec![],
        }
    }

    pub fn render(&mut self) {
        self.resources.load_texture("test.png");
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
        }
        for sprite in &self.sprites {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, sprite.texture);
            }
        }
    }
}
