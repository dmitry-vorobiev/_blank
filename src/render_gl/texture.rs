use gl;

use resources::Resources;

pub struct Texture {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Texture {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Texture, String> {
        use gl::types::{GLint, GLuint, GLvoid};
        use image::GenericImageView;

        let img = res.load_image(name).unwrap();

        let mut texture_id: GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut texture_id);
            gl.BindTexture(gl::TEXTURE_2D, texture_id);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as GLint,
                img.width() as GLint,
                img.height() as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.raw_pixels().as_ptr() as *const GLvoid
            );
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture {
            gl: gl.clone(),
            id: texture_id,
        })
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0 + (slot as gl::types::GLenum));
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &mut self.id);
        }
    }
}