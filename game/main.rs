use gl;
use glfw;
use gl::types::*;

use game::engine::Engine;
use game::program::Program;
use game::shader::{Shader, VertexShaderType, FragmentShaderType};
use game::vertexarray::VertexArray;

static VS_SRC: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(0.0, 0.0, 0.0, 1.0);\n\
    }";

static VERTEX_DATA: [GLfloat, ..6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5,
];


fn make_vertex_array() -> VertexArray {
    let mut va = VertexArray::new();
    va.set_data(VERTEX_DATA);
    va
}

fn make_program() -> ~Program {
    let mut prog = do Program::new |linker| {
        linker.attach(Shader::new(VS_SRC, VertexShaderType).unwrap());
        linker.attach(Shader::new(FS_SRC, FragmentShaderType).unwrap());
    }.unwrap();
    prog.bind_frag_data_location("out_color", 0);
    prog.enable_vertex_array("position", 2, gl::FLOAT);
    prog
}

pub fn main() {
    let e = Engine::new();
    do e.run |window| {
        let va = make_vertex_array();
        let prog = make_program();

        while !window.should_close() {
            glfw::poll_events();

            gl::ClearColor(0.3, 0.6, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            prog.bind();
            va.draw(gl::TRIANGLES, 0, 3);

            window.swap_buffers();
        }
    };
}
