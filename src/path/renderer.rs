use std::ffi::CString;

use super::path_painter::PathPainter;

use super::generated_gl as gl;

pub struct Renderer {
    pub(crate) triangle_program: gl::types::GLuint,
    pub(crate) curve_program: gl::types::GLuint,
    pub(crate) quad_curve_stroke_program: gl::types::GLuint,
}

impl Renderer {
    pub fn new() -> Self {
        let triangle_program = create_program(
            include_str!("../../data/shaders/triangle.vert"),
            include_str!("../../data/shaders/triangle.frag"),
        );
        let curve_program = create_program(
            include_str!("../../data/shaders/curve.vert"),
            include_str!("../../data/shaders/curve.frag"),
        );

        let quad_curve_stroke_program = create_program(
            include_str!("../../data/shaders/quad_curve_stroke.vert"),
            include_str!("../../data/shaders/quad_curve_stroke.frag"),
        );

        Self {
            triangle_program,
            curve_program,
            quad_curve_stroke_program,
        }
    }

    pub fn draw_path(&self, path_painter: &mut impl PathPainter) {
        path_painter.draw(&self);
    }
}

fn create_program(vert_shader_code: &str, frag_shader_code: &str) -> gl::types::GLuint {
    unsafe {
        let shader_vertex = gl::CreateShader(gl::VERTEX_SHADER);
        let shader_source =
            CString::new(vert_shader_code).expect("Vertex shader to be valid c-string");
        gl::ShaderSource(shader_vertex, 1, &shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_vertex);

        let mut buffer = [0i8; 8192];
        let mut buffer_size = 0;
        gl::GetShaderInfoLog(shader_vertex, 8192, &mut buffer_size, buffer.as_mut_ptr());
        if buffer_size > 0 {
            println!(
                "Vertex shader error: {}",
                String::from_utf8_lossy(buffer.map(|v| v as u8).as_slice())
            );
        }

        let shader_fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        let shader_source =
            CString::new(frag_shader_code).expect("Fragment shader to be valid c-string");
        gl::ShaderSource(
            shader_fragment,
            1,
            &shader_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(shader_fragment);

        gl::GetShaderInfoLog(shader_fragment, 8192, &mut buffer_size, buffer.as_mut_ptr());
        if buffer_size > 0 {
            println!(
                "Fragment shader error: {}",
                String::from_utf8_lossy(buffer.map(|v| v as u8).as_slice())
            );
        }

        let program = gl::CreateProgram();
        gl::AttachShader(program, shader_vertex);
        gl::AttachShader(program, shader_fragment);
        gl::LinkProgram(program);

        gl::GetProgramInfoLog(program, 8192, &mut buffer_size, buffer.as_mut_ptr());
        if buffer_size > 0 {
            println!(
                "Program error: {}",
                String::from_utf8_lossy(buffer.map(|v| v as u8).as_slice())
            );
        }

        program
    }
}
