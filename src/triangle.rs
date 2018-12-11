use gl;
use failure;
use render_gl::{self, buffer, data};
use resources::Resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::VertexBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vertices: [Vertex; 3] = [
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into()
            }, // bottom right
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into()
            }, // bottom left
            Vertex {
                pos: (0.0,  0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into()
            }  // top
        ];

        let vao = buffer::VertexArray::new(gl);
        vao.bind();

        let vbo = buffer::VertexBuffer::new(gl);
        vbo.bind();

        vbo.static_draw_data(&vertices);
        Vertex::vertex_attrib_pointers(gl);
        vao.unbind();
        vbo.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.bind();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}