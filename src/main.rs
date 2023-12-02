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
extern crate hex_io as io;

#[cfg(not(feature = "std"))]
use {alloc::vec::Vec, math::num::Real};
use {
    gfx::{glsl, mesh::Mesh, program::Program, uniform::Uniform, Draw},
    math::{
        constants::{
            pos::{ORIGIN, X, Y, Z},
            rgb::{BLACK, BLUE, GREEN, RED, WHITE},
        },
        geometry::Primitive as GeoPrim,
        matrix::{look_at, orthographic, perspective},
        vector::R3,
    },
    io::{
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

    let grid: Vec<Mesh> = (-5..=5)
        .map(|z| {
            let a = X * 5.0 + Z * z as f32;
            let b = X * -5.0 + Z * z as f32;

            GeoPrim::Line(a, b)
        })
        .chain((-5..=5).map(|x| {
            let a = Z * 5.0 + X * x as f32;
            let b = Z * -5.0 + X * x as f32;

            GeoPrim::Line(a, b)
        }))
        .map(|line| Mesh::from(line))
        .collect();

    let axes = [
        Mesh::from(GeoPrim::Line(ORIGIN, X)),
        Mesh::from(GeoPrim::Line(ORIGIN, Y)),
        Mesh::from(GeoPrim::Line(ORIGIN, Z)),
    ];

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
        for line in &grid {
            line.draw(None);
        }

        RED.bind(2);
        axes[0].draw(None);

        GREEN.bind(2);
        axes[1].draw(None);

        BLUE.bind(2);
        axes[2].draw(None);

        window.swap();
        window.delay(1);
    }
}
