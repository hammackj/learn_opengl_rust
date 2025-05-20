extern crate nalgebra_glm as glm;

use gl::types::GLint;
use gl::{self};
use glfw::ffi::glfwGetTime;
use glfw::Context;
use glfw::{self};
use shader::Shader;

pub mod shader;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct State {
    pub wireframe: bool,
    pub camera_position: glm::Vec3,
    pub delta_time: f32,
    pub last_frame: f32,
    pub first_mouse: bool,
    pub yaw: f32,
    pub pitch: f32,
    pub last_mouse_x: f32,
    pub last_mouse_y: f32,
    pub fov: f32,
    pub camera_front: glm::Vec3,
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
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    //window.set_scroll_callback(scroll_callback);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    window.set_framebuffer_size_callback(glfw_framebuffer_size_callback);

    let mut state = State {
        wireframe: false,
        camera_position: glm::vec3(0.0, 0.0, 3.0),
        delta_time: 0.0,
        last_frame: 0.0,
        first_mouse: true,
        yaw: -90.0,
        pitch: 0.0,
        last_mouse_x: SCR_WIDTH as f32 / 2.0,
        last_mouse_y: SCR_HEIGHT as f32 / 2.0,
        fov: 45.0,
        camera_front: glm::vec3(0.0, 0.0, -1.0),
    };

    state.wireframe = false;

    let mut shader = Shader::new("assets/shaders/vertex.vs", "assets/shaders/fragment.fs");

    #[rustfmt::skip]
    let vertices = [
        -0.5f32, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    let cube_positions = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let mut vao = 0;
    let mut vbo = 0;

    let mut texture: u32 = 0;
    let mut face_texture: u32 = 0;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

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
            5 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as i32,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        let image_data = image::open("assets/textures/container.jpg").unwrap();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            image_data.width() as GLint,
            image_data.height() as GLint,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            image_data.as_bytes().as_ptr() as *const _,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::GenTextures(1, &mut face_texture);
        gl::BindTexture(gl::TEXTURE_2D, face_texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        let mut face_data = image::open("assets/textures/awesomeface.png").unwrap();
        face_data = face_data.flipv();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            face_data.width() as GLint,
            face_data.height() as GLint,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            face_data.as_bytes().as_ptr() as *const _,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        shader.use_program();

        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event, &mut state);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, face_texture);

            shader.use_program();

            let projection;

            // Note the glm rust port has these paramaters wrong lol
            projection = glm::perspective(
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                state.fov.to_radians(),
                0.1,
                100.0,
            );

            shader.set_mat4("projection", projection);

            let view;

            let camera_up: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);

            view = glm::look_at(
                &state.camera_position,
                &(state.camera_position + &state.camera_front),
                &camera_up,
            );

            shader.set_mat4("view", view);

            gl::BindVertexArray(vao);

            for (i, cube_position) in cube_positions.iter().enumerate() {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, cube_position);

                let mut angle: f32 = 20.0 * i as f32;

                if i % 3 == 0 {
                    angle = 20.0 * glfwGetTime() as f32;
                }

                model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));

                shader.set_mat4("model", model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

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

    match event {
        glfw::WindowEvent::CursorPos(x_position, y_position) => {
            handle_mouse(state, x_position, y_position);
        }
        glfw::WindowEvent::Scroll(x_offset, y_offset) => {
            handle_mouse_scroll(state, x_offset, y_offset);
        }
        glfw::WindowEvent::Key(key, _scancode, action, _mods) => match (key, action) {
            (Key::Escape, Action::Press) => {
                window.set_should_close(true);
            }
            (Key::F, Action::Press) => {
                handle_wireframe(state);
            }
            (Key::W | Key::A | Key::D | Key::S, Action::Press) => {
                handle_movement(state, key);
            }
            _ => {}
        },
        _ => {}
    }
}

fn glfw_framebuffer_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

fn handle_movement(state: &mut State, key: glfw::Key) {
    let camera_front: glm::Vec3 = glm::vec3(0.0, 0.0, -1.0);
    let camera_up: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);

    let current_frame: f32 = unsafe { glfwGetTime() as f32 };
    state.delta_time = current_frame - state.last_frame;
    state.last_frame = current_frame;

    let camera_speed: f32 = 2.5 * state.delta_time;

    if key == glfw::Key::W {
        state.camera_position += camera_speed * camera_front;
    }

    if key == glfw::Key::S {
        state.camera_position -= camera_speed * camera_front;
    }

    if key == glfw::Key::A {
        state.camera_position -=
            glm::normalize(&glm::cross(&camera_front, &camera_up)) * camera_speed;
    }

    if key == glfw::Key::D {
        state.camera_position +=
            glm::normalize(&glm::cross(&camera_front, &camera_up)) * camera_speed;
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

fn handle_mouse(state: &mut State, x_position: f64, y_position: f64) {
    if state.first_mouse {
        state.last_mouse_x = x_position as f32;
        state.last_mouse_y = y_position as f32;
        state.first_mouse = false;
    }

    let mut x_offset = (x_position as f32) - state.last_mouse_x;
    let mut y_offset = state.last_mouse_y - (y_position as f32);

    state.last_mouse_x = x_position as f32;
    state.last_mouse_y = y_position as f32;

    let sensitivity: f32 = 0.1;
    x_offset *= sensitivity;
    y_offset *= sensitivity;

    state.yaw += x_offset;
    state.pitch += y_offset;

    if state.pitch > 89.0 {
        state.pitch = 89.0;
    }

    if state.pitch < -89.0 {
        state.pitch = -89.0;
    }

    let mut direction = glm::vec3(0.0, 0.0, 0.0);
    direction.x = state.yaw.to_radians().cos() * state.pitch.to_radians().cos();
    direction.y = state.pitch.to_radians().sin();
    direction.z = state.yaw.to_radians().sin() * state.pitch.to_radians().cos();

    state.camera_front = glm::normalize(&direction);
}

fn handle_mouse_scroll(state: &mut State, _x_offset: f64, y_offset: f64) {
    state.fov -= y_offset as f32;

    if state.fov < 1.0 {
        state.fov = 1.0;
    }

    if state.fov > 45.0 {
        state.fov = 45.0;
    }

    println!("FOV = {}", state.fov);
}

//fn scroll_callback(window: &mut glfw::Window)

fn gl_last_error() {
    unsafe {
        let errno = gl::GetError();

        if errno != gl::NO_ERROR {
            println!("Last GL Error = {}", errno);
        }
    }
}
