use gl::types::GLfloat;
use std::vec::Vec;

pub struct Mesh {
    texture: i32, // TODO sort out resource IDs
    tris: Vec<GLfloat>,
}
