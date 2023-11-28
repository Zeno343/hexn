use {crate::sys::*, core::mem::size_of};

pub type BufferId = GLuint;

#[derive(Debug)]
pub struct Buffer {
    id: BufferId,
    target: GLenum,
    len: usize,
}

impl Buffer {
    pub fn new(target: Target) -> Self {
        unsafe {
            let mut id = 0;
            glGenBuffers(1, &mut id);

            Self {
                id,
                len: 0,
                target: target as GLenum,
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindBuffer(self.target, self.id);
        }
    }

    pub fn buffer_data<Data>(&mut self, data: &[Data], usage: Usage) {
        unsafe {
            glBufferData(
                self.target,
                (size_of::<Data>() * data.len()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage as GLenum,
            );
        }

        self.len = data.len();
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Target {
    Array = GL_ARRAY_BUFFER,
    ElementArray = GL_ELEMENT_ARRAY_BUFFER,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Usage {
    StaticDraw = GL_STATIC_DRAW,
}
