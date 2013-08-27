use gl;
use gl::types::*;

use std::cast::transmute;
use std::sys::size_of;

pub struct VertexArray {
    id: GLuint,
    priv vbo: GLuint,
}

impl VertexArray {

    pub fn new() -> VertexArray {
        let vao = 0;
        let vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1, &vbo);
        }
        VertexArray { id: vao, vbo: vbo }
    }

    pub fn set_data(&mut self, data: &[GLfloat]) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * size_of::<GLfloat>()) as GLsizeiptr,
                           transmute(&data[0]),
                           gl::STATIC_DRAW);
        }
    }

    pub fn draw(&self, ty: GLenum, first: int, count: int) {
        gl::BindVertexArray(self.id);
        gl::DrawArrays(ty, first as GLint, count as GLint);
    }
}

impl Drop for VertexArray {
    fn drop(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.id)
        };
    }
}
