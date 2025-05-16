use gl;
use glfw;
use glfw::Context;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::Resizable(true));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));

    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    window.set_framebuffer_size_callback(glfw_framebuffer_size_callback);

    // Note slightly different than https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/1.2.hello_window_clear/hello_window_clear.cpp
    // As the glfw crate seems to do event slightly different
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}

// This would be the process input function in the C++ version
fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

fn glfw_framebuffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
