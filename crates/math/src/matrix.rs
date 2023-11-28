use super::vector::{Vector, R3};

pub type Mat4 = [f32; 16];

pub fn perspective(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
    [
        2.0 * near / (right - left),
        0.0,
        (right + left) / (right - left),
        0.0,
        0.0,
        2.0 * near / (top - bottom),
        (top + bottom) / (top - bottom),
        0.0,
        0.0,
        0.0,
        -(far + near) / (far - near),
        -2.0 * far * near / (far - near),
        0.0,
        0.0,
        -1.0,
        0.0,
    ]
}

pub fn orthographic(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
    [
        2.0 / (right - left),
        0.0,
        0.0,
        -(right + left) / (right - left),
        0.0,
        2.0 / (top - bottom),
        0.0,
        -(top + bottom) / (top - bottom),
        0.0,
        0.0,
        -2.0 / (far - near),
        -(far + near) / (far - near),
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}
pub fn look_at(from: R3, to: R3) -> Mat4 {
    let up: R3 = [0.0, 1.0, 0.0].into();
    let z = (from - to).norm();
    let x = up.cross(z).norm();
    let y = z.cross(x);

    let t_x = -x.dot(from);
    let t_y = -y.dot(from);
    let t_z = -z.dot(from);

    [
        x[0], y[0], z[0], 0.0, x[1], y[1], z[1], 0.0, x[2], y[2], z[2], 0.0, t_x, t_y, t_z, 1.0,
    ]
}
