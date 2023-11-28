#![cfg_attr(
    not(feature = "std"),
    no_std,
    no_main,
    allow(internal_features),
    feature(lang_items),
    feature(naked_functions)
)]

#[cfg(not(feature = "std"))]
mod no_std;

#[cfg(not(feature = "std"))]
extern crate alloc;
extern crate hex_gfx as gfx;
extern crate hex_math as math;
extern crate hex_win as win;

#[cfg(not(feature = "std"))]
use {alloc::vec::Vec, math::num::Real};
use {
    gfx::{
        glsl,
        mesh::{Mesh, Topology},
        program::Program,
        uniform::Uniform,
        Draw,
    },
    math::{
        constants::{
            pos::{ORIGIN, X, Y, Z},
            rgb::{BLACK, BLUE, GREEN, RED, WHITE},
        },
        matrix::{look_at, orthographic, perspective},
        vector::{span, R3},
    },
    win::{
        event::{KeyCode, WindowEvent},
        time, Window,
    },
};

const WINDOW_RES: [i32; 2] = [800, 600];
const WINDOW_NAME: &str = "hexen v0.0.1\0";

const MVP: &str = glsl!("shaders/mvp.vert");
const RGB: &str = glsl!("shaders/rgb_uniform.frag");

const PI: f32 = 3.14159;
const PITCH: f32 = 30.0 * PI / 180.0;

#[cfg_attr(not(feature = "std"), no_mangle)]
fn main() {
    #[cfg(feature = "std")]
    println!("hexen v0.0.1");
    let window = Window::new(WINDOW_NAME, WINDOW_RES).expect("window creation failed");

    let shader = Program::new(MVP, RGB).unwrap();

    let half_w = 5u32;
    let lattice = &span([X, Z], half_w);

    let w = 1 + 2 * half_w;
    let mut idcs = Vec::with_capacity(w.pow(2) as usize);
    for u in 0..w {
        for v in 0..w {
            if u < w - 1 {
                let u0 = u;
                let u1 = u0 + 1;

                idcs.push(u0 * w + v);
                idcs.push(u1 * w + v);
            }

            if v < w - 1 {
                let v0 = v;
                let v1 = v0 + 1;

                idcs.push(u * w + v0);
                idcs.push(u * w + v1);
            }
        }
    }

    let grid = Mesh::new().with_array(lattice).with_idcs(&idcs);
    let frame = Mesh::new().with_array(&[ORIGIN, X, Y, Z]);

    let projs = {
        let aspect = WINDOW_RES[1] as f32 / WINDOW_RES[0] as f32;
        [
            perspective(-1.0, 1.0, aspect, -aspect, 1.0, 100.0),
            orthographic(-1.0, 1.0, aspect, -aspect, 1.0, 100.0),
        ]
    };

    let mut current_proj = 0;
    let start = time();
    'main: loop {
        let elapsed = time() - start;
        for event in window.events() {
            match event {
                WindowEvent::Quit => {
                    break 'main;
                }

                WindowEvent::Keyboard {
                    down: true,
                    keycode: KeyCode::Tab,
                    ..
                } => current_proj = (current_proj + 1) % 2,

                _ => {}
            }
        }

        window.clear_color([BLACK[0], BLACK[1], BLACK[2], 1.0]);
        shader.bind();

        let t = elapsed / 5_000.0;
        let view = look_at(
            R3([2.0 * t.sin(), 2.0 * PITCH.sin(), 2.0 * t.cos()]),
            R3([0.0; 3]),
        );
        view.bind(0);
        projs[current_proj].bind(1);

        WHITE.bind(2);
        grid.draw_idx(Topology::Lines, None);

        RED.bind(2);
        frame.draw_idx(Topology::Lines, Some(&[0, 1]));

        GREEN.bind(2);
        frame.draw_idx(Topology::Lines, Some(&[0, 2]));

        BLUE.bind(2);
        frame.draw_idx(Topology::Lines, Some(&[0, 3]));

        window.swap();
        window.delay(1);
    }
}
