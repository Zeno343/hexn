#![no_std]
#![feature(core_intrinsics)]

extern crate alloc;
extern crate hex_io as io;
extern crate hex_math as math;

pub mod buffer;
pub mod mesh;
pub mod program;
pub mod shader;
pub mod uniform;
pub mod vertex;

use sys::*;

pub trait Draw {
    fn clear_color(&self, rgba: [f32; 4]);
}

impl Draw for io::Window {
    fn clear_color(&self, [r, g, b, a]: [f32; 4]) {
        unsafe {
            glClearColor(r, g, b, a);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    }
}

#[allow(
    dead_code,
    improper_ctypes,
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types
)]
mod sys {
    include! {concat!(env!("OUT_DIR"), "/gl.rs")}
}
