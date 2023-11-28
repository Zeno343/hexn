use crate::{
    shader::{Shader, Stage},
    sys::*,
};

#[derive(Debug)]
pub struct Program(GLuint);

impl Program {
    pub fn new(vert_src: &str, frag_src: &str) -> Result<Program, &'static str> {
        unsafe {
            let prog = glCreateProgram();
            let vert = Shader::new(vert_src, Stage::Vertex)?;
            let frag = Shader::new(frag_src, Stage::Fragment)?;

            glAttachShader(prog, vert.id);
            glAttachShader(prog, frag.id);
            glLinkProgram(prog);

            Ok(Program(prog))
        }
    }

    pub fn bind(&self) {
        unsafe {
            glUseProgram(self.0);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            glDeleteProgram(self.0);
        }
    }
}
