use gl;
use std::ffi::{CString, CStr};

use resources::{self, Resources};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Unable to load resource {}", name)]
    ResourceLoad {
        name: String,
        #[cause] inner: resources::Error
    },

    #[fail(display = "Unable to determine shader type for resource {}", name)]
    UnknownShaderType {
        name: String
    },

    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError {
        name: String,
        message: String
    },

    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError {
        name: String,
        message: String
    },
}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        use gl::types::{GLchar, GLint, GLuint};

        let program_id: GLuint = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()); }
        }

        unsafe { gl.LinkProgram(program_id); }

        let mut success: GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let err_msg: CString = whitespace_c_str(len as usize);
            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    err_msg.as_ptr() as *mut GLchar
                );
            }
            return Err(err_msg.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()); }
        }

        Ok(Program { gl: gl.clone(), id: program_id })
    }

    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert.shader",
            ".frag.shader",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                Shader::from_res(gl, res, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..])
            .map_err(|message| Error::LinkError {
                name: name.into(),
                message
            })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.UseProgram(0);
        }
    }

    pub fn set_uniform1i(&self, name: &str, val: i32) {
        let u_loc = self.uniform_location(name);

        unsafe {
            self.gl.Uniform1i(u_loc, val as gl::types::GLint);
        }
    }

    fn uniform_location(&self, name: &str) -> gl::types::GLint {
        use gl::types::{GLchar, GLint};

        let location: GLint = unsafe {
            let c_name = CStr::from_bytes_with_nul_unchecked(name.as_bytes());
            self.gl.GetUniformLocation(
                self.id,
                c_name.as_ptr() as *mut GLchar
            )
        };

        if location != -1 {
            println!("[OPENGL WARN]: uniform '{}' doesn't exist", name);
        }

        location
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(gl: &gl::Gl, source: &CStr, shader_type: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(&gl, source, shader_type)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vert.shader", gl::VERTEX_SHADER),
            (".frag.shader", gl::FRAGMENT_SHADER),
        ];

        let shader_type = POSSIBLE_EXT.iter()
            .find(|&&(extension, _)| name.ends_with(extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::UnknownShaderType { name: name.into() })?;

        let source = res.load_cstring(name)
            .map_err(|e| Error::ResourceLoad {
                name: name.into(),
                inner: e
            })?;

        Shader::from_source(gl, &source, shader_type)
            .map_err(|message| Error::CompileError {
                name: name.into(),
                message
            })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    shader_type: gl::types::GLuint
) -> Result<gl::types::GLuint, String> {
    use gl::types::{GLchar, GLint, GLuint};

    let id: GLuint = unsafe { gl.CreateShader(shader_type) };
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let err_msg: CString = whitespace_c_str(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                err_msg.as_ptr() as *mut GLchar
            );
        }
        return Err(err_msg.to_string_lossy().into_owned());
    }

    Ok(id)
}

pub fn whitespace_c_str(len: usize) -> std::ffi::CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));

    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
}