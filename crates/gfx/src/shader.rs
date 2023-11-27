use crate::sys::*;
pub type ShaderId = GLuint;

pub struct Shader {
    pub id: ShaderId,
}

impl Shader {
    pub fn new(source: &str, stage: Stage) -> Result<Shader, &'static str> {
        unsafe {
            let id = glCreateShader(stage as GLenum);
            glShaderSource(
                id as ShaderId,
                1,
                &(source.as_ptr() as _),
                core::ptr::null(),
            );
            glCompileShader(id);

            let mut success = GL_FALSE as GLint;
            glGetShaderiv(id, GL_COMPILE_STATUS, &mut success);

            match success as GLuint {
                GL_TRUE => Ok(Shader { id }),

                GL_FALSE => Err("shader compile failed"),

                _ => Err("unknown shader error"),
            }
        }
    }

    pub fn attach(&self, prog: GLuint) {
        unsafe {
            glAttachShader(prog, self.id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            glDeleteShader(self.id);
        }
    }
}

#[repr(u32)]
pub enum Stage {
    Vertex = GL_VERTEX_SHADER,
    Fragment = GL_FRAGMENT_SHADER,
}

#[macro_export]
macro_rules! glsl {
    ( $source:literal ) => {
        concat!(include_str!($source), "\0")
    };
}
