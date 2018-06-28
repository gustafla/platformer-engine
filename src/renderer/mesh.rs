use gl::types::GLfloat;
use std::vec::Vec;

pub struct Mesh {
    pub texture: i32, // TODO sort out resource IDs
    pub tris: Vec<GLfloat>,
}
