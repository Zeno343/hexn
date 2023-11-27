use crate::{math::vector::R3, sys::*};

pub trait Uniform {
    fn bind(&self, location: i32);
}

impl Uniform for f32 {
    fn bind(&self, location: i32) {
        unsafe {
            glUniform1f(location, *self);
        }
    }
}

impl Uniform for [i32; 2] {
    fn bind(&self, location: i32) {
        unsafe {
            glUniform2i(location, self[0], self[1]);
        }
    }
}

impl Uniform for R3 {
    fn bind(&self, location: i32) {
        let vec = self.0;

        unsafe {
            glUniform3f(location, vec[0], vec[1], vec[2]);
        }
    }
}

impl Uniform for [f32; 16] {
    fn bind(&self, location: i32) {
        unsafe {
            glUniformMatrix4fv(location, 1, GL_FALSE as GLboolean, self.as_ptr());
        }
    }
}
