use {crate::sys::*, math::vector::R3};

pub trait Vertex {
    fn bind(location: u32);
}

impl Vertex for [f32; 2] {
    fn bind(location: u32) {
        unsafe {
            const SIZE: GLint = 2;
            const TYPE: GLenum = GL_FLOAT;
            const NORM: GLboolean = GL_FALSE as _;
            const STRIDE: GLsizei = 0;
            const OFFSET: *const GLvoid = core::ptr::null();

            glVertexAttribPointer(location, SIZE, TYPE, NORM, STRIDE, OFFSET);
            glEnableVertexAttribArray(location);
        }
    }
}

impl Vertex for R3 {
    fn bind(location: u32) {
        unsafe {
            const SIZE: GLint = 3;
            const TYPE: GLenum = GL_FLOAT;
            const NORM: GLboolean = GL_FALSE as _;
            const STRIDE: GLsizei = 0;
            const OFFSET: *const GLvoid = core::ptr::null();

            glVertexAttribPointer(location, SIZE, TYPE, NORM, STRIDE, OFFSET);
            glEnableVertexAttribArray(location);
        }
    }
}
