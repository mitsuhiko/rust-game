use gl;
use gl::types::*;

use std::ptr::null;

use game::shader::Shader;

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new() -> Program {
        Program { id: gl::CreateProgram() }
    }

    pub fn new_with_shaders(vs: &Shader, fs: &Shader) -> Program {
        let mut rv = Program::new();
        rv.attach_shader(vs);
        rv.attach_shader(fs);
        rv.link();
        rv
    }

    pub fn attach_shader(&mut self, shader: &Shader) {
        gl::AttachShader(self.id, shader.id);
    }

    pub fn link(&mut self) {
        gl::LinkProgram(self.id);
    }

    pub fn bind_frag_data_location(&mut self, name: &str, location: int) {
        unsafe {
            do name.with_c_str |ptr| {
                gl::BindFragDataLocation(self.id, location as u32, ptr);
            }
        }
    }

    pub fn enable_vertex_array(&mut self, name: &str, size: int, ty: GLenum) {
        unsafe {
            let pos_attr = do name.with_c_str |ptr| {
                gl::GetAttribLocation(self.id, ptr)
            };
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, size as GLint, ty,
                                    gl::FALSE as GLboolean, 0,
                                    null());
        }
    }

    pub fn bind(&self) {
        gl::UseProgram(self.id);
    }
}

impl Drop for Program {
    fn drop(&self) {
        gl::DeleteProgram(self.id);
    }
}
