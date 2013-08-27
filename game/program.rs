use gl;
use gl::types::*;
use std::hashmap::HashMap;
use std::vec;
use std::ptr::null;

use game::shader::Shader;
use game::utils::charbuf_to_str;


pub struct ProgramError {
    msg: ~str,
}

impl ProgramError {
    fn new(msg: ~str) -> ~ProgramError {
        ~ProgramError { msg: msg }
    }
}

impl ToStr for ~ProgramError {
    #[inline]
    fn to_str(&self) -> ~str {
        fmt!("Program error: %s", self.msg)
    }
}

/// Holds location information for a uniform or attribute as
/// well as the type.
pub struct Location {
    id: GLuint,
    ty: GLenum,
}

/// Encapsulates an OpenGL program.
struct Program {
    id: GLuint,
    priv shaders: ~[~Shader],
    priv attribs: HashMap<~str, Location>,
    priv uniforms: HashMap<~str, Location>,
}


/// Holds multiple shaders that work together.  This object is created
/// with the helper of the Linker.
impl Program {

    /// Constructor that creates a new program.  The passed closure is given
    /// a linker which can be used to attach different shaders to the
    /// program.
    pub fn new(f: &fn(&mut Linker)) -> Result<~Program, ~ProgramError> {
        let mut linker = Linker::new();
        f(linker);
        linker.link()
    }

    /// Binds the program.
    pub fn bind(&self) {
        gl::UseProgram(self.id);
    }

    /// Unbinds the program.
    pub fn unbind() {
        gl::UseProgram(0);
    }

    /// Returns a reference to an attribute location, if it exists.
    pub fn get_attrib<'a>(&'a self, name: &str) -> Option<&'a Location> {
        self.attribs.find(&name.to_owned())
    }

    /// Returns a reference to an uniform location, if it exists.
    pub fn get_uniform<'a>(&'a self, name: &str) -> Option<&'a Location> {
        self.uniforms.find(&name.to_owned())
    }

    /// Binds the frag data location for a given attribute.
    pub fn bind_frag_data_location(&mut self, name: &str, location: int) {
        unsafe {
            do name.with_c_str |ptr| {
                gl::BindFragDataLocation(self.id, location as u32, ptr);
            }
        }
    }

    /// Binds the attrib pointer for a given attribute.
    pub fn enable_vertex_array(&mut self, name: &str, size: int, ty: GLenum) {
        unsafe {
            let loc = self.get_attrib(name).unwrap();
            gl::EnableVertexAttribArray(loc.id);
            gl::VertexAttribPointer(loc.id as GLuint, size as GLint, ty,
                                    gl::FALSE as GLboolean, 0,
                                    null());
        }
    }
}

impl Drop for Program {
    fn drop(&self) {
        for shader in self.shaders.iter() {
            gl::DetachShader(self.id, shader.id);
        }
        gl::DeleteProgram(self.id);
    }
}


/// The linker struct temporarily holds the program's id and the list
/// of shaders that will be attached.  Once the shaders are moved out
/// the Drop will be disabled.
struct Linker {
    id: GLuint,
    priv shaders: Option<~[~Shader]>,
}

/// The linker is a helper object that is used to create a program.  It does
/// not expose much of a public interface besides the `attach` method.
impl Linker {
    fn new() -> ~Linker {
        ~Linker {
            id: gl::CreateProgram(),
            shaders: Some(~[]),
        }
    }

    /// Attach a shader to the program.
    pub fn attach(&mut self, shader: ~Shader) {
        let id = shader.id;
        self.shaders.get_mut_ref().push(shader);
        gl::AttachShader(self.id, id);
    }

    /// Consumes the unlinked shader program, returning a rv program or
    /// an error log if the linking failed.
    fn link(&mut self) -> Result<~Program, ~ProgramError> {
        let shaders = self.shaders.take_unwrap();

        gl::LinkProgram(self.id);

        // handle errors
        let status = gl::FALSE as GLint;
        unsafe { gl::GetProgramiv(self.id, gl::LINK_STATUS, &status) };
        if status != (gl::TRUE as GLint) {
            unsafe {
                let len: GLint = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &len);

                let buf = vec::from_elem(len as uint, 0u8);
                gl::GetProgramInfoLog(self.id, len, null(),
                                      vec::raw::to_ptr(buf) as *GLchar);

                return Err(ProgramError::new(charbuf_to_str(buf)));
            }
        }

        // The program has been successfully rv, so cast it accordingly
        let mut rv = ~Program {
            id: self.id,
            shaders: shaders,
            attribs: HashMap::new(),
            uniforms: HashMap::new(),
        };

        // Get all attributes
        unsafe {
            let max_attr_len = 0;
            let num_attribs = 0;
            gl::GetProgramiv(rv.id, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH, &max_attr_len);
            gl::GetProgramiv(rv.id, gl::ACTIVE_ATTRIBUTES, &num_attribs);
            let buf = vec::from_elem(max_attr_len as uint, 0u8);

            for id in range(0, num_attribs as GLuint) {
                let (len, size, ty) = (0, 0, 0);
                gl::GetActiveAttrib(rv.id, id, max_attr_len, &len, &size, &ty,
                                    vec::raw::to_ptr(buf) as *GLchar);
                rv.attribs.insert(
                    charbuf_to_str(buf),
                    Location { id: id, ty: ty }
                );
            }
        }

        // Get all uniforms
        unsafe {
            let max_uniform_len = 0;
            let num_uniforms = 0;
            gl::GetProgramiv(rv.id, gl::ACTIVE_UNIFORM_MAX_LENGTH, &max_uniform_len);
            gl::GetProgramiv(rv.id, gl::ACTIVE_UNIFORMS, &num_uniforms);
            let buf = vec::from_elem(max_uniform_len as uint, 0u8);

            for id in range(0, num_uniforms as GLuint) {
                let (len, size, ty) = (0, 0, 0);
                gl::GetActiveUniform(rv.id, id, max_uniform_len, &len, &size, &ty,
                                     vec::raw::to_ptr(buf) as *GLchar);
                rv.attribs.insert(
                    charbuf_to_str(buf),
                    Location { id: id, ty: ty }
                );
            }
        }

        Ok(rv)
    }
}

/// The linker's drop only executes for as long as it did not link
/// anything.  After that all the data is moved to the program.
impl Drop for Linker {
    fn drop(&self) {
        match self.shaders {
            None => { return; }
            Some(ref shaders) => {
                for shader in shaders.iter() {
                    gl::DetachShader(self.id, shader.id);
                }
                gl::DeleteProgram(self.id);
            }
        }
    }
}
