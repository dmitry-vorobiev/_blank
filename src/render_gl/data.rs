use gl;
use gl::types::{GLint, GLuint, GLvoid};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            d0, d1, d2
        }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as GLuint);
        gl.VertexAttribPointer(
            location as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as GLint,
            offset as *const GLvoid
        );
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    pub inner: ::vec_2_10_10_10::Vector,
}

impl u2_u10_u10_u10_rev_float {
    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as GLuint);
        gl.VertexAttribPointer(
            location as GLuint,
            4,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            gl::TRUE,
            stride as GLint,
            offset as *const GLvoid
        );
    }
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3)
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_ {
    pub d0: i8,
}

impl i8_ {
    pub fn new(d0: i8) -> i8_ {
        i8_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as GLuint);
        gl.VertexAttribIPointer(
            location as GLuint,
            1,
            gl::BYTE,
            stride as GLint,
            offset as *const GLvoid
        );
    }
}

impl From<i8> for i8_ {
    fn from(other: i8) -> Self {
        i8_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_float {
    pub d0: i8,
}

impl i8_float {
    pub fn new(d0: i8) -> i8_float {
        i8_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as GLuint);
        gl.VertexAttribPointer(
            location as GLuint,
            1,
            gl::BYTE,
            gl::TRUE,
            stride as GLint,
            offset as *const GLvoid
        );
    }
}

impl From<i8> for i8_float {
    /// Create this data type from i8
    fn from(other: i8) -> Self {
        i8_float::new(other)
    }
}
