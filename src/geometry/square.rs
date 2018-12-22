use gl;
use failure;
use nalgebra as na;
use render_gl::{self, buffer};
use resources::Resources;
use geometry::vertex::Vertex;

pub struct Square {
    model_matrix: na::Matrix4<f32>,
    program: render_gl::Program,
    _vbo: buffer::VertexBuffer,
    _ibo: buffer::ElementArray,
    vao: buffer::VertexArray,
}

impl Square {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Square, failure::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vertices: [Vertex; 4] = [
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into()
            }, // bottom right
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into()
            }, // bottom left
            Vertex {
                pos: (-0.5,  0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into()
            },  // top left
            Vertex {
                pos: (0.5,  0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into()
            },  // top right
        ];

        let indices: [u8; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        let vao = buffer::VertexArray::new(gl);
        vao.bind();

        let vbo = buffer::VertexBuffer::new(gl);
        vbo.bind();

        let ibo = buffer::ElementArray::new(gl);
        ibo.bind();

        ibo.static_draw_data(&indices);
        vbo.static_draw_data(&vertices);
        Vertex::vertex_attrib_pointers(gl);
        vao.unbind();
        vbo.unbind();
        ibo.unbind();

        Ok(Square {
            model_matrix: na::Matrix4::new_scaling(0.5),
            program,
            _vbo: vbo,
            _ibo: ibo,
            vao,
        })
    }

    pub fn update_pos(&mut self, vec: &na::Vector3<f32>) {
        self.model_matrix = self.model_matrix.append_translation(vec);
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.bind();
        self.program.set_uniform_mat4f("ModelMatrix\0", &self.model_matrix);

        self.vao.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_BYTE,
                0 as *const gl::types::GLvoid
            );
        }
    }
}