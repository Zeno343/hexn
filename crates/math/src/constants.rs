pub mod pos {
    use crate::vector::R3;

    pub const ORIGIN: R3 = R3([0.0, 0.0, 0.0]);
    pub const X: R3 = R3([1.0, 0.0, 0.0]);
    pub const Y: R3 = R3([0.0, 1.0, 0.0]);
    pub const Z: R3 = R3([0.0, 0.0, 1.0]);
}

pub mod rgb {
    use crate::vector::R3;

    pub const RED: R3 = R3([1.0, 0.0, 0.0]);
    pub const GREEN: R3 = R3([0.0, 1.0, 0.0]);
    pub const BLUE: R3 = R3([0.0, 0.0, 1.0]);
    pub const WHITE: R3 = R3([1.0, 1.0, 1.0]);
    pub const BLACK: R3 = R3([0.0, 0.0, 0.0]);
}
