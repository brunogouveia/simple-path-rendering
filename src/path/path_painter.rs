use glam::Vec2;

use super::mesh::Mesh;
use super::path::Path;
use super::primitives::QuadCurve;
use super::renderer::Renderer;

use super::generated_gl as gl;

pub trait PathPainter {
    fn draw(&mut self, renderer: &Renderer);
}

pub struct FillPathPainter {
    pub base_mesh: Mesh,
    pub curve_mesh: Mesh,
}

impl FillPathPainter {
    pub fn new(mut path: Path) -> Self {
        let mut base_mesh_vertices = Vec::new();
        let mut curve_mesh_vertices = Vec::new();

        for sub_path in path.sub_paths.iter_mut() {
            for idx in 2..sub_path.points.len() {
                base_mesh_vertices.push(sub_path.points[0]);
                base_mesh_vertices.push(sub_path.points[idx - 1]);
                base_mesh_vertices.push(sub_path.points[idx]);
            }

            for quad_curve in sub_path.quad_curves.iter() {
                curve_mesh_vertices.push(quad_curve.c0);
                curve_mesh_vertices.push(quad_curve.c1);
                curve_mesh_vertices.push(quad_curve.c2);
            }
        }

        Self {
            base_mesh: Mesh::new(base_mesh_vertices),
            curve_mesh: Mesh::new(curve_mesh_vertices),
        }
    }
}

impl PathPainter for FillPathPainter {
    fn draw(&mut self, renderer: &Renderer) {
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
}

pub struct StrokePathPainter {
    pub base_mesh: Mesh,
    quad_curve_meshes: Vec<(QuadCurve, Mesh)>,
    pub width: f32,
}

impl StrokePathPainter {
    pub fn new(mut path: Path, width: f32) -> Self {
        let mut base_mesh_vertices = Vec::new();
        let mut quad_curve_meshes = Vec::new();

        for sub_path in path.sub_paths.iter_mut() {
            for idx in 2..sub_path.points.len() {
                base_mesh_vertices.push(sub_path.points[0]);
                base_mesh_vertices.push(sub_path.points[idx - 1]);
                base_mesh_vertices.push(sub_path.points[idx]);
            }

            for quad_curve in sub_path.quad_curves.iter() {
                let mut quad_curve_mesh_vertices = Vec::new();

                let mut min_x = f32::MAX;
                let mut min_y = f32::MAX;
                let mut max_x = f32::MIN;
                let mut max_y = f32::MIN;

                let points = [quad_curve.c0, quad_curve.c1, quad_curve.c2];
                for point in points {
                    min_x = min_x.min(point.x);
                    max_x = max_x.max(point.x);

                    min_y = min_y.min(point.y);
                    max_y = max_y.max(point.y);
                }

                min_x -= 5.0 * width;
                min_y -= 5.0 * width;
                max_x += 5.0 * width;
                max_y += 5.0 * width;

                quad_curve_mesh_vertices.push(Vec2::new(min_x, min_y));
                quad_curve_mesh_vertices.push(Vec2::new(min_x, max_y));
                quad_curve_mesh_vertices.push(Vec2::new(max_x, max_y));

                quad_curve_mesh_vertices.push(Vec2::new(min_x, min_y));
                quad_curve_mesh_vertices.push(Vec2::new(max_x, min_y));
                quad_curve_mesh_vertices.push(Vec2::new(max_x, max_y));

                quad_curve_meshes.push((quad_curve.clone(), Mesh::new(quad_curve_mesh_vertices)));
            }
        }

        Self {
            base_mesh: Mesh::new(base_mesh_vertices),
            quad_curve_meshes,
            width,
        }
    }
}

impl PathPainter for StrokePathPainter {
    fn draw(&mut self, renderer: &Renderer) {
        unsafe {
            gl::Disable(gl::STENCIL_TEST);

            gl::UseProgram(renderer.quad_curve_stroke_program);
            check_log_error();

            gl::Uniform4f(3, 1.0, 0.0, 0.0, 1.0);
            gl::Uniform1f(4, self.width);

            for (quad_curve, quad_curve_mesh) in self.quad_curve_meshes.iter() {
                let c0 = quad_curve.c0;
                let c1 = quad_curve.c1;
                let c2 = quad_curve.c2;

                gl::Uniform2f(0, c0.x, c0.y);
                gl::Uniform2f(1, c1.x, c1.y);
                gl::Uniform2f(2, c2.x, c2.y);

                quad_curve_mesh.bind();
                check_log_error();

                gl::DrawArrays(gl::TRIANGLES, 0, quad_curve_mesh.size());
                check_log_error();
            }
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
