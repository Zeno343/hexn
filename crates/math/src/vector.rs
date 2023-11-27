use {
    crate::num::Real,
    alloc::vec::Vec,
    core::ops::{Add, Div, Mul, Sub},
};

pub trait Vector<F: Field>:
    Mul<F, Output = Self> + Add<Self, Output = Self> + Sized + Copy
{
    fn len(self) -> F;
    fn dot(self, rhs: Self) -> F;
    fn norm(self) -> Self;
    fn cross(self, rhs: Self) -> Self;
}

pub trait Field: Add + Sub + Div + Mul + Sized {}

impl<F: Add<F> + Sub<F> + Mul<F> + Div<F> + Sized> Field for F {}

#[derive(Clone, Copy)]
pub struct R3(pub [f32; 3]);

impl Vector<f32> for R3 {
    fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    fn dot(self, rhs: Self) -> f32 {
        let a = self.0;
        let b = rhs.0;

        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    fn norm(self) -> Self {
        self / self.len()
    }

    fn cross(self, rhs: Self) -> Self {
        let a = self.0;
        let b = rhs.0;

        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
        .into()
    }
}

pub fn span<V: Vector<f32>>(basis: [V; 2], range: u32) -> Vec<V> {
    let range = range as i32;

    let mut span = Vec::new();
    for y in -range..=range {
        for x in -range..=range {
            span.push(basis[0] * x as f32 + basis[1] * y as f32)
        }
    }

    span
}

impl core::ops::Add for R3 {
    type Output = Self;

    fn add(self, rhs: R3) -> Self {
        let a = self.0;
        let b = rhs.0;

        [a[0] + b[0], a[1] + b[1], a[2] + b[2]].into()
    }
}

impl core::ops::Neg for R3 {
    type Output = Self;

    fn neg(self) -> R3 {
        let a = self.0;
        [-a[0], -a[1], -a[2]].into()
    }
}

impl core::ops::Sub for R3 {
    type Output = Self;

    fn sub(self, rhs: R3) -> R3 {
        self + -rhs
    }
}

impl core::ops::Div<f32> for R3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        let a = self.0;

        [a[0] / rhs, a[1] / rhs, a[2] / rhs].into()
    }
}

impl core::ops::Mul<f32> for R3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let a = self.0;

        [a[0] * rhs, a[1] * rhs, a[2] * rhs].into()
    }
}

impl core::ops::Index<u8> for R3 {
    type Output = f32;

    fn index(&self, idx: u8) -> &Self::Output {
        &self.0[idx as usize]
    }
}

impl From<[f32; 3]> for R3 {
    fn from(r3: [f32; 3]) -> Self {
        Self(r3)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let x: R3 = [1.0, 2.0, 3.0].into();
        let y: R3 = [4.0, 5.0, 6.0].into();

        assert_eq!(x + y, [5.0, 7.0, 9.0].into())
    }

    #[test]
    fn sub() {
        let x: R3 = [1.0, 0.0, 0.0].into();
        let y: R3 = [1.0, 0.0, 0.0].into();

        assert_eq!(x - y, [0.0, 0.0, 0.0].into())
    }

    #[test]
    fn orthogonal_dot_product() {
        let x: R3 = [1.0, 0.0, 0.0].into();
        let y: R3 = [0.0, 1.0, 0.0].into();

        assert_eq!(x.dot(y), 0.0)
    }

    #[test]
    fn unit_len() {
        let x: R3 = [1.0, 0.0, 0.0].into();
        assert_eq!(x.len(), 1.0)
    }

    #[test]
    fn norm() {
        let x: R3 = [2.0, 0.0, 0.0].into();
        assert_eq!(x.norm(), [1.0, 0.0, 0.0].into())
    }
}
