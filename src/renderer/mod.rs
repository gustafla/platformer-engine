mod sprite;
mod mesh;
mod resources;

use std::vec::Vec;
use gl::{ActiveTexture, BindTexture, TEXTURE0, TEXTURE_2D};
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
        self.resources.load_texture("test.flif");
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
