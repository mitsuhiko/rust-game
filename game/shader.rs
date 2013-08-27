use gl;
use gl::types::*;

use std::io;
use std::vec;
use std::ptr::null;
use std::path::Path;

use game::utils::charbuf_to_str;


pub struct ShaderError {
    msg: ~str,
}

impl ShaderError {
    fn new(msg: ~str) -> ~ShaderError {
        ~ShaderError { msg: msg }
    }
}

impl ToStr for ~ShaderError {
    #[inline]
    fn to_str(&self) -> ~str {
        fmt!("Shader error: %s", self.msg)
    }
}


#[deriving(Eq)]
pub enum ShaderType {
    VertexShaderType,
    TessControlShaderType,
    TessEvaluationShaderType,
    GeometryShaderType,
    FragmentShaderType
}

impl ShaderType {
    pub fn gl_enum(self) -> GLenum {
        match self {
            VertexShaderType => gl::VERTEX_SHADER,
            TessControlShaderType => gl::TESS_CONTROL_SHADER,
            TessEvaluationShaderType => gl::TESS_EVALUATION_SHADER,
            GeometryShaderType => gl::GEOMETRY_SHADER,
            FragmentShaderType => gl::FRAGMENT_SHADER,
        }
    }
}


/// Holds shader information.  This should be linked against
/// a program after it has been created.
pub struct Shader {
    id: GLuint,
    ty: ShaderType,
}

impl Shader {

    /// Creates a new shader of a given type from GLSL source.
    pub fn new(src: &str, ty: ShaderType) -> Result<~Shader, ~ShaderError> {
        let id = gl::CreateShader(ty.gl_enum());
        unsafe {
            let status = gl::FALSE as GLint;

            do src.with_c_str |ptr| {
                gl::ShaderSource(id, 1, &ptr, null());
            }

            gl::CompileShader(id);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &status);

            if status != (gl::TRUE as GLint) {
                let len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &len);
                let buf = vec::from_elem(len as uint, 0u8);
                gl::GetShaderInfoLog(id, len, null(),
                                     vec::raw::to_ptr(buf) as *GLchar);
                gl::DeleteShader(id);
                Err(ShaderError::new(charbuf_to_str(buf)))
            } else {
                Ok(~Shader { id: id, ty: ty })
            }
        }
    }

    /// Loads a shader from a file.
    pub fn new_from_file(path: &Path, ty: ShaderType) -> Result<~Shader, ~ShaderError> {
        match io::read_whole_file_str(path) {
            Err(msg) => Err(ShaderError::new(msg)),
            Ok(src) => Shader::new(src, ty)
        }
    }
}

impl Drop for Shader {
    fn drop(&self) {
        gl::DeleteShader(self.id);
    }
}
