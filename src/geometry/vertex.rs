use render_gl::data;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    #[location = "0"]
    pub pos: data::f32_f32_f32,
    #[location = "1"]
    pub clr: data::u2_u10_u10_u10_rev_float,
}