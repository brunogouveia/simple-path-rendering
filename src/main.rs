mod path;

use path::{generated_gl as gl, path_painter::StrokePathPainter};

use glam::Vec2;
// use glft;
use glfw::{
    Action, Context, Key, OpenGlProfileHint, SwapInterval, Window, WindowEvent, WindowHint,
    WindowMode,
};
use path::{path::Path, path_painter::FillPathPainter, renderer::Renderer};

// use optick;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersionMajor(4));
    glfw.window_hint(WindowHint::ContextVersionMinor(1));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Samples(Some(8)));

    let (mut window, event_receiver) = glfw
        .create_window(1024, 768, "Simple example", WindowMode::Windowed)
        .expect("Unable to create window");

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    gl::load_with(|s| glfw.get_proc_address_raw(s));

    glfw.set_swap_interval(SwapInterval::Sync(1));

    unsafe {
        let renderer = Renderer::new();

        let mut path = Path::new();

        path.line_to(Vec2::new(100.0, 100.0));
        path.line_to(Vec2::new(100.0, 600.0));
        path.line_to(Vec2::new(600.0, 600.0));
        path.line_to(Vec2::new(600.0, 100.0));

        path.move_to(Vec2::new(600.0, 600.0));
        path.quadratic_curve_to(Vec2::new(100.0, 600.0), Vec2::new(100.0, 100.0));

        path.close_subpath();

        let mut start_path = Path::new();

        start_path.line_to(Vec2::new(100.0, 100.0));
        start_path.line_to(Vec2::new(600.0, 400.0));
        start_path.line_to(Vec2::new(100.0, 400.0));
        start_path.line_to(Vec2::new(600.0, 100.0));
        start_path.line_to(Vec2::new(350.0, 600.0));
        start_path.close_subpath();

        let mut stroke_path_painter = StrokePathPainter::new(path.clone(), 5.0);

        let mut path_painter = FillPathPainter::new(start_path);

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Enable(gl::STENCIL_TEST);
        gl::Enable(gl::DEPTH_TEST);
        gl::CullFace(gl::FRONT_AND_BACK);

        gl::DepthFunc(gl::ALWAYS);
        gl::StencilFunc(gl::ALWAYS, 1, 0xFF);

        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::SAMPLE_SHADING);
        gl::MinSampleShading(1.0);

        let mut rounded_rect_path = Path::new();
        create_rounded_corner_rect_path(&mut rounded_rect_path, 100.0, 100.0, 360.0, 200.0, 50.0);

        let mut rounded_corner_path_painter = FillPathPainter::new(rounded_rect_path);

        let mut rounded_sep_rect_path = Path::new();
        create_rounded_separate_corner_rect_path(
            &mut rounded_sep_rect_path,
            100.0,
            100.0,
            360.0,
            200.0,
            50.0,
            10.0,
            20.0,
            5.0,
        );

        let mut rounded_sep_corner_path_painter = FillPathPainter::new(rounded_sep_rect_path);

        let mut rounded_border_path = Path::new();
        create_rounded_corner_rect_path(&mut rounded_border_path, 100.0, 100.0, 360.0, 200.0, 50.0);

        create_rounded_corner_rect_path(&mut rounded_border_path, 110.0, 110.0, 340.0, 180.0, 40.0);

        let mut rounded_border_path_painter = FillPathPainter::new(rounded_border_path);

        while !window.should_close() {
            {
                glfw.poll_events();
                for (_, event) in glfw::flush_messages(&event_receiver) {
                    handle_window_event(&mut window, event);
                }
            }

            let (width, height) = window.get_framebuffer_size();
            gl::Viewport(0, 0, width, height);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            // renderer.draw_path(&mut rounded_corner_path_painter);
            // renderer.draw_path(&mut path_painter);
            // renderer.draw_path(&mut stroke_path_painter);
            // renderer.draw_path(&mut rounded_sep_corner_path_painter);
            renderer.draw_path(&mut rounded_border_path_painter);

            window.swap_buffers()
        }

        drop(window);
    }
}

fn handle_window_event(window: &mut Window, event: WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

fn create_rounded_corner_rect_path(
    rounded_rect_path: &mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    corner_radius: f32,
) {
    let offset = Vec2::new(x, y);

    rounded_rect_path.line_to(offset + Vec2::new(0.0, corner_radius));
    rounded_rect_path.line_to(offset + Vec2::new(0.0, height - corner_radius));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(0.0, height),
        offset + Vec2::new(corner_radius, height),
    );
    rounded_rect_path.line_to(offset + Vec2::new(width - corner_radius, height));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(width, height),
        offset + Vec2::new(width, height - corner_radius),
    );
    rounded_rect_path.line_to(offset + Vec2::new(width, corner_radius));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(width, 0.0),
        offset + Vec2::new(width - corner_radius, 0.0),
    );
    rounded_rect_path.line_to(offset + Vec2::new(corner_radius, 0.0));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(0.0, 0.0),
        offset + Vec2::new(0.0, corner_radius),
    );
    rounded_rect_path.close_subpath();
}

fn create_rounded_separate_corner_rect_path(
    rounded_rect_path: &mut Path,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    tl_corner_radius: f32,
    tr_corner_radius: f32,
    bl_corner_radius: f32,
    br_corner_radius: f32,
) {
    let offset = Vec2::new(x, y);

    rounded_rect_path.line_to(offset + Vec2::new(0.0, bl_corner_radius));
    rounded_rect_path.line_to(offset + Vec2::new(0.0, height - tl_corner_radius));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(0.0, height),
        offset + Vec2::new(tl_corner_radius, height),
    );
    rounded_rect_path.line_to(offset + Vec2::new(width - tr_corner_radius, height));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(width, height),
        offset + Vec2::new(width, height - tr_corner_radius),
    );
    rounded_rect_path.line_to(offset + Vec2::new(width, br_corner_radius));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(width, 0.0),
        offset + Vec2::new(width - br_corner_radius, 0.0),
    );
    rounded_rect_path.line_to(offset + Vec2::new(bl_corner_radius, 0.0));
    rounded_rect_path.quadratic_curve_to(
        offset + Vec2::new(0.0, 0.0),
        offset + Vec2::new(0.0, bl_corner_radius),
    );
    rounded_rect_path.close_subpath();
}
