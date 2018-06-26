mod sprite;
mod mesh;

use std::vec::Vec;
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
        // TODO implement
    }
}
