use glam::Vec2;

use super::generated_gl as gl;

#[derive(Default, Debug)]
pub struct Mesh {
    vertex_array_object_name: gl::types::GLuint,
    buffer_name: gl::types::GLuint,
    vertices: Vec<Vec2>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        if vertices.len() == 0 {
            return Self::default();
        }

        let mut vertex_array_object_name = 0;
        let mut buffer_name = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array_object_name);
            gl::BindVertexArray(vertex_array_object_name);

            gl::GenBuffers(1, &mut buffer_name);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_name);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<Vec2>() * vertices.len()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

            gl::BindVertexArray(0);
        }

        Self {
            vertex_array_object_name,
            buffer_name,
            vertices,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object_name);
        }
    }

    pub fn size(&self) -> i32 {
        self.vertices.len() as i32
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer_name);
            gl::DeleteVertexArrays(1, &self.vertex_array_object_name);
        }
    }
}
