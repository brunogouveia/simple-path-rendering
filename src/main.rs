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

        // Create VAO
        // let mut vertex_array_object: gl::types::GLuint = 0;
        // gl::GenVertexArrays(1, &mut vertex_array_object);
        // gl::BindVertexArray(vertex_array_object);

        // let positions = vec![
        //     Vec2::new(0.0, 0.0),
        //     Vec2::new(100.0, 0.0),
        //     Vec2::new(0.0, 100.0),
        // ];

        let mut path = Path::new();

        // path_builder.line_to(Vec2::new(100.0, 100.0));
        // path_builder.line_to(Vec2::new(100.0, 200.0));
        // path_builder.line_to(Vec2::new(200.0, 200.0));
        // path_builder.curve_to(Vec2::new(220.0, 150.0), Vec2::new(100.0, 100.0));
        // path_builder.curve_to(Vec2::new(20.0, 150.0), Vec2::new(100.0, 200.0));
        // path_builder.close();

        // path_builder.line_to(Vec2::new(100.0, 100.0));
        // path_builder.line_to(Vec2::new(100.0, 600.0));
        // path_builder.line_to(Vec2::new(600.0, 600.0));
        // path_builder.line_to(Vec2::new(0.0, 300.0));

        path.line_to(Vec2::new(100.0, 100.0));
        path.line_to(Vec2::new(100.0, 600.0));
        path.line_to(Vec2::new(600.0, 600.0));
        path.line_to(Vec2::new(600.0, 100.0));

        path.move_to(Vec2::new(600.0, 600.0));
        path.quadratic_curve_to(Vec2::new(100.0, 600.0), Vec2::new(100.0, 100.0));
        // path.move_to(Vec2::new(600.0, 600.0));
        // path.quadratic_curve_to(Vec2::new(100.0, 600.0), Vec2::new(100.0, 100.0));

        path.close_subpath();

        let mut stroke_path_painter = StrokePathPainter::new(path.clone(), 5.0);

        // let mut path = Path::new();
        // path.line_to(Vec2::new(100.0, 100.0));
        // path.line_to(Vec2::new(100.0, 600.0));
        // path.line_to(Vec2::new(600.0, 600.0));
        // path.close_subpath();

        // path.move_to(Vec2::new(600.0, 600.0));
        // path.curve_to(Vec2::new(220.0, 150.0), Vec2::new(100.0, 100.0));
        // path.curve_to(Vec2::new(20.0, 150.0), Vec2::new(100.0, 200.0));
        // path.close_subpath();
        let mut path_painter = FillPathPainter::new(path);

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Enable(gl::STENCIL_TEST);
        gl::Enable(gl::DEPTH_TEST);
        gl::CullFace(gl::FRONT_AND_BACK);

        gl::DepthFunc(gl::ALWAYS);
        gl::StencilFunc(gl::ALWAYS, 1, 0xFF);

        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::SAMPLE_SHADING);
        gl::MinSampleShading(1.0);

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

            // renderer.draw_path(&mut stroke_path_painter);
            renderer.draw_path(&mut path_painter);

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
