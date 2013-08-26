use gl;
use gl::types::*;
use std::ptr::null;

pub struct Shader {
    id: GLuint,
    ty: GLenum,
}

impl Shader {
    pub fn new(src: &str, ty: GLenum) -> Shader {
        let shader = gl::CreateShader(ty);
        unsafe {
            do src.with_c_str |ptr| {
                gl::ShaderSource(shader, 1, &ptr, null());
            }
        }
        gl::CompileShader(shader);
        Shader { id: shader, ty: ty }
    }
}

impl Drop for Shader {
    fn drop(&self) {
        gl::DeleteShader(self.id);
    }
}
