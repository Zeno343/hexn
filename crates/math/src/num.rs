use core::intrinsics::{cosf32, sinf32, sqrtf32};

pub trait Real {
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

impl Real for f32 {
    fn sqrt(self) -> Self {
        unsafe { sqrtf32(self) }
    }

    fn sin(self) -> Self {
        unsafe { sinf32(self) }
    }

    fn cos(self) -> Self {
        unsafe { cosf32(self) }
    }
}
