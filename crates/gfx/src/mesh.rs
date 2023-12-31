use {
    crate::{
        buffer::{Buffer, Target, Usage},
        sys::*,
        vertex::Vertex,
    },
    alloc::vec::Vec,
    core::ptr,
};

pub type MeshId = GLuint;

#[derive(Debug)]
pub struct Mesh {
    id: MeshId,
    arrays: Vec<Buffer>,
    idcs: Option<Buffer>,
}

impl Mesh {
    pub fn new() -> Self {
        unsafe {
            let mut id = 0;
            glGenVertexArrays(1, &mut id);

            Self {
                id,
                arrays: Vec::new(),
                idcs: None,
            }
        }
    }

    fn bind(&self) {
        unsafe {
            glBindVertexArray(self.id);
        }
    }

    pub fn with_array<V: Vertex>(mut self, verts: &[V]) -> Self {
        if self.arrays.len() > 0 {
            assert_eq!(self.arrays[0].len(), verts.len())
        }

        let mut buffer = Buffer::new(Target::Array);
        buffer.bind();
        buffer.buffer_data(verts, Usage::StaticDraw);

        self.bind();
        V::bind(self.arrays.len() as u32);
        self.arrays.push(buffer);
        self
    }

    pub fn with_idcs(mut self, idcs: &[u32]) -> Self {
        self.bind();
        self.idcs = Some({
            let mut buffer = Buffer::new(Target::ElementArray);
            buffer.bind();
            buffer.buffer_data(idcs, Usage::StaticDraw);

            buffer
        });
        self
    }

    pub fn draw_idx(&self, topo: Topology, idcs: Option<&[u32]>) {
        self.bind();
        unsafe {
            if let Some(idcs) = idcs {
                glDrawElements(
                    topo as GLenum,
                    idcs.len() as GLint,
                    GL_UNSIGNED_INT,
                    idcs.as_ptr() as *const GLvoid,
                );
            } else {
                glDrawElements(
                    topo as GLenum,
                    match &self.idcs {
                        Some(idcs) => idcs.len(),
                        None => 0,
                    } as GLint,
                    GL_UNSIGNED_INT,
                    ptr::null() as *const GLvoid,
                );
            }
        }
    }

    pub fn draw(&self, topo: Topology) {
        self.bind();
        unsafe {
            glDrawArrays(topo as GLenum, 0, self.arrays[0].len() as GLint);
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Topology {
    Triangles = GL_TRIANGLES,
    TriFan = GL_TRIANGLE_FAN,
    TriStrip = GL_TRIANGLE_STRIP,
    Lines = GL_LINES,
    Points = GL_POINTS,
}
