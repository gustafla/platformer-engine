use gl::types::GLuint;

pub struct Sprite {
    pub texture: GLuint,
    pub position: (f32, f32),
    pub scale: (f32, f32),
}
