use std::vec::Vec;

mod sprite;
mod mesh;
use renderer::sprite::Sprite;
use renderer::mesh::Mesh;

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
