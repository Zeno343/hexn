#![no_std]
extern crate alloc;
pub mod event;
use {alloc::ffi::CString, core::ffi::CStr, event::WindowEvents, sys::*};

pub struct Window {
    window: *mut SDL_Window,
    ctx: SDL_GLContext,
}

impl Window {
    pub fn new(name: &str, [w, h]: [i32; 2]) -> Result<Self, CString> {
        unsafe {
            SDL_InitSubSystem(SDL_INIT_VIDEO);

            let window = SDL_CreateWindow(
                name.as_ptr().cast(),
                SDL_WINDOWPOS_UNDEFINED_MASK as Sint32,
                SDL_WINDOWPOS_UNDEFINED_MASK as Sint32,
                w,
                h,
                (SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN) as Uint32,
            );

            if !window.is_null() {
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 6);
                SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 16);

                let ctx = SDL_GL_CreateContext(window);
                SDL_GL_MakeCurrent(window, ctx);

                Ok(Self { window, ctx })
            } else {
                let err = CString::new(CStr::from_ptr(SDL_GetError()).to_bytes())
                    .expect("failed to retrieve error");
                SDL_QuitSubSystem(SDL_INIT_VIDEO);
                Err(err)
            }
        }
    }

    pub fn events(&self) -> WindowEvents {
        WindowEvents::new()
    }

    pub fn delay(&self, ms: u32) {
        unsafe {
            SDL_Delay(ms);
        }
    }

    pub fn swap(&self) {
        unsafe {
            SDL_GL_SwapWindow(self.window);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DeleteContext(self.ctx);
            SDL_QuitSubSystem(SDL_INIT_VIDEO);
        }
    }
}

pub fn time() -> f32 {
    unsafe { SDL_GetTicks() as f32 }
}

#[allow(
    dead_code,
    improper_ctypes,
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types
)]
mod sys {
    include! {concat!(env!("OUT_DIR"), "/sdl.rs")}
}
