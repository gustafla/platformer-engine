use gl::types::GLuint;

pub struct Sprite {
    texture: GLuint,
    position: (f32, f32),
    scale: (f32, f32),
}
