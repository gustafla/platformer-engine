extern crate gl;
extern crate png;

use std;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use std::ffi::{CString, CStr};
use gl::types::{GLuint, GLint, GLchar};

pub struct Resources {
    textures: Vec<GLuint>,
    shaders: Vec<GLuint>,
}

fn is_pot(width: f32, height: f32) -> bool {
    width.log2().ceil() == width.log2().floor()
        && height.log2().ceil() == height.log2().floor()
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            textures: Vec::with_capacity(1024),
            shaders: Vec::with_capacity(1024),
        }
    }

    pub fn load_texture(&mut self, filename: &str) -> Result<u32, String> {
        let file = File::open(filename).unwrap();
        let png_decoder = png::Decoder::new(file);
        let (png_info, mut png_reader) = png_decoder.read_info().unwrap();
        if !is_pot(png_info.width as f32, png_info.height as f32) {
            return Err(format!("Texture {} size is not a power of two", filename));
        }
        let mut image = vec![0; png_info.buffer_size()];
        png_reader.next_frame(&mut image).unwrap();

        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            //TODO
        }

        Ok(0)
    }

    pub fn load_shader(&mut self, filename: &str) -> Result<u32, String> {
        // Try loading the file
        let mut file = File::open(filename).unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        let src = CString::new(string).unwrap();

        // Match file extension to determine what kind of shader to generate
        let kind = match filename.split(".").last() {
            Some("frag") => gl::FRAGMENT_SHADER,
            Some("vert") => gl::VERTEX_SHADER,
            _ => return Err(format!("Can't determine shader type for {}", filename)),
        };

        // Generate and compile shader
        let id = unsafe {gl::CreateShader(kind)};
        unsafe {
            gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        // Check compilation status
        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error: CString = unsafe {CString::from_vec_unchecked(buffer)};
            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }
            return Err(error.to_string_lossy().into_owned());
        }

        self.shaders.push(id);
        Ok(self.shaders.len() as u32)
    }
}
