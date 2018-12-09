use gl::{self, types::{GLuint, GLvoid, GLsizeiptr}};

pub struct VertexBuffer {
    gl: gl::Gl,
    vbo: gl::types::GLuint,
}

impl VertexBuffer {
    pub fn new(gl: &gl::Gl) -> VertexBuffer {
        let mut vbo: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        VertexBuffer {
            gl: gl.clone(),
            vbo
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * ::std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct VertexArray {
    gl: gl::Gl,
    vao: GLuint,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> VertexArray {
        let mut vao: GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            gl: gl.clone(),
            vao
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &mut self.vao);
        }
    }
}