use gl;
use glfw;
use glfw::Context;
use shader::Shader;

pub mod shader;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct State {
    pub wireframe: bool,
}

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

    let mut state = State { wireframe: false };

    state.wireframe = false;

    let mut shader = Shader::new("assets/shaders/vertex.vs", "assets/shaders/fragment.fs");

    #[rustfmt::skip]
    let vertices: [f32; 18] = [
    // positions         // colors
     0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
    -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom let
     0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
    ];

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<f32>() as i32,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        shader.use_program();
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event, &mut state);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindVertexArray(0);

            gl_last_error();
        }

        window.swap_buffers();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        shader.delete_program();
    }
}

// This would be the process input function in the C++ version
fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent, state: &mut State) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        Event::Key(Key::F, _, Action::Press, _) => {
            handle_wireframe(state);
        }
        _ => {}
    }
}

fn glfw_framebuffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

fn handle_wireframe(state: &mut State) {
    if state.wireframe {
        state.wireframe = false;
    } else {
        state.wireframe = true;
    }

    if state.wireframe {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) };
    } else {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL) };
    }
}

fn gl_last_error() {
    unsafe {
        let errno = gl::GetError();

        if errno != gl::NO_ERROR {
            println!("Last GL Error = {}", errno);
        }
    }
}
