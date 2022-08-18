use super::mesh::Mesh;
use super::path::Path;
use super::renderer::Renderer;

use super::generated_gl as gl;

pub struct PathPainter {
    pub base_mesh: Mesh,
    pub curve_mesh: Mesh,
}

impl PathPainter {
    pub fn new(mut path: Path) -> Self {
        let mut base_mesh_vertices = Vec::new();
        let mut curve_mesh_vertices = Vec::new();

        for sub_path in path.sub_paths.iter_mut() {
            for idx in 2..sub_path.points.len() {
                base_mesh_vertices.push(sub_path.points[0]);
                base_mesh_vertices.push(sub_path.points[idx - 1]);
                base_mesh_vertices.push(sub_path.points[idx]);
            }

            curve_mesh_vertices.append(&mut sub_path.curve_points);
        }

        Self {
            base_mesh: Mesh::new(base_mesh_vertices),
            curve_mesh: Mesh::new(curve_mesh_vertices),
        }
    }

    pub fn draw(&mut self, renderer: &Renderer) {
        unsafe {
            gl::Enable(gl::STENCIL_TEST);
            gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);

            gl::StencilFuncSeparate(gl::FRONT, gl::ALWAYS, 0, 0);
            gl::StencilFuncSeparate(gl::BACK, gl::ALWAYS, 0, 0);
            // gl::StencilMask(0xFF);
            // gl::StencilOp(gl::INCR, gl::INCR, gl::INCR);
            gl::StencilOpSeparate(gl::FRONT, gl::DECR, gl::DECR, gl::DECR);
            gl::StencilOpSeparate(gl::BACK, gl::INCR, gl::INCR, gl::INCR);

            // gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            // gl::StencilMask(0xFF);
            // gl::StencilOpSeparate(gl::FRONT, gl::INCR, gl::INCR, gl::INCR);
            // gl::StencilOpSeparate(gl::BACK, gl::DECR, gl::DECR, gl::DECR);

            // Stencil phase
            gl::UseProgram(renderer.triangle_program);
            check_log_error();

            self.base_mesh.bind();
            check_log_error();

            gl::DrawArrays(gl::TRIANGLES, 0, self.base_mesh.size());

            if self.curve_mesh.size() > 0 {
                gl::UseProgram(renderer.curve_program);
                check_log_error();

                self.curve_mesh.bind();
                check_log_error();

                gl::DrawArrays(gl::TRIANGLES, 0, self.curve_mesh.size());
            }

            // Cover phase
            gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
            gl::StencilFunc(gl::LEQUAL, 1, 0xFF);
            // TODO: replace with zero
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);

            // Triangles
            gl::UseProgram(renderer.triangle_program);
            check_log_error();

            self.base_mesh.bind();
            check_log_error();
            gl::DrawArrays(gl::TRIANGLES, 0, self.base_mesh.size());

            // Curve
            gl::UseProgram(renderer.curve_program);
            check_log_error();

            self.curve_mesh.bind();
            check_log_error();

            gl::DrawArrays(gl::TRIANGLES, 0, self.curve_mesh.size());
            check_log_error();
        }
    }

    pub fn draw_sroke(&mut self, renderer: &Renderer) {
        unsafe {
            gl::UseProgram(renderer.quad_curve_stroke_program);
            check_log_error();

            self.base_mesh.bind();
            check_log_error();

            gl::DrawArrays(gl::TRIANGLES, 0, self.base_mesh.size());
            check_log_error();
        }
    }
}

fn check_log_error() {
    unsafe {
        let err = gl::GetError();

        let err_msg = match err {
            1 => "",
            gl::NO_ERROR => "",
            gl::INVALID_ENUM => "Invalid enum",
            gl::INVALID_VALUE => "Invalid value",
            gl::INVALID_OPERATION => "Invalid operation",
            gl::STACK_OVERFLOW => "Stack overflow",
            gl::STACK_UNDERFLOW => "Stack underflow",
            gl::OUT_OF_MEMORY => "Out of memory",
            _ => "Unknown error",
        };

        if err_msg.len() > 0 {
            println!("OpenGL error: {}", err_msg);
        }
    }
}
