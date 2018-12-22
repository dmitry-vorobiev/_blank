use gl::{self, types::{GLuint, GLvoid, GLsizeiptr}};

pub trait BufferType {
    const BUFFER_TYPE: GLuint;
}

pub struct VertexBufferType;
impl BufferType for VertexBufferType {
    const BUFFER_TYPE: GLuint = gl::ARRAY_BUFFER;
}

pub struct ElementArrayType;
impl BufferType for ElementArrayType {
    const BUFFER_TYPE: GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub type VertexBuffer = Buffer<VertexBufferType>;
pub type ElementArray = Buffer<ElementArrayType>;

pub struct Buffer<B> where B: BufferType {
    gl: gl::Gl,
    vbo: GLuint,
    _marker: ::std::marker::PhantomData<B>,
}

impl<B> Buffer<B> where B: BufferType {
    pub fn new(gl: &gl::Gl) -> Buffer<B> {
        let mut vbo: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }

        Buffer {
            gl: gl.clone(),
            vbo,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                B::BUFFER_TYPE,
                (data.len() * ::std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl<B> Drop for Buffer<B> where B: BufferType {
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