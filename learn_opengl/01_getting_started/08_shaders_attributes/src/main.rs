use gl;
use glfw;
use glfw::Context;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERT_SHADER_SOURCE: &str = "#version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 ourColor;
     
    void main()
    {
        gl_Position = vec4(aPos, 1.0);
        ourColor = aColor;
    }";

const FRAG_SHADER_SOURCE: &str = "#version 330 core
    out vec4 FragColor;
    in vec3 ourColor;

    void main()
    {
        FragColor = vec4(ourColor, 1.0f);
    }";

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

    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(
            vertex_shader,
            1,
            &VERT_SHADER_SOURCE.as_bytes().as_ptr().cast(),
            &VERT_SHADER_SOURCE.len().try_into().unwrap(),
        );
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0_i32;
            let mut v: Vec<u8> = Vec::with_capacity(512);
            gl::GetShaderInfoLog(vertex_shader, 512, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED: {}",
                String::from_utf8_lossy(&v)
            );
        }
    }

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(
            fragment_shader,
            1,
            &FRAG_SHADER_SOURCE.as_bytes().as_ptr().cast(),
            &FRAG_SHADER_SOURCE.len().try_into().unwrap(),
        );
        gl::CompileShader(fragment_shader);

        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(512);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 512, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED: {}",
                String::from_utf8_lossy(&v)
            );
        }
    }

    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!(
                "ERROR::SHADER::PROGRAM::LINKING_FAILED: {}",
                String::from_utf8_lossy(&v)
            );
        }

        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

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
        gl::UseProgram(shader_program);
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
        gl::DeleteProgram(shader_program);
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
