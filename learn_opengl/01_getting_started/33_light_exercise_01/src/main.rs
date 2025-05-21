extern crate nalgebra_glm as glm;

use camera::Camera;
use gl::{self};
use glfw::ffi::{glfwGetKey, glfwGetTime};
use glfw::Context;
use glfw::{self};
use shader::Shader;

pub mod camera;
pub mod shader;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct State {
    pub wireframe: bool,

    pub delta_time: f32,
    pub last_frame: f32,
    pub first_mouse: bool,

    pub last_mouse_x: f32,
    pub last_mouse_y: f32,

    pub light_position: glm::Vec3,
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
    window.set_sticky_keys(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    window.set_framebuffer_size_callback(glfw_framebuffer_size_callback);

    let mut state = State {
        wireframe: false,
        delta_time: 0.0,
        last_frame: 0.0,
        first_mouse: true,
        last_mouse_x: SCR_WIDTH as f32 / 2.0,
        last_mouse_y: SCR_HEIGHT as f32 / 2.0,
        light_position: glm::vec3(1.2, 1.0, 2.0),
    };

    let mut camera = Camera::new(
        glm::vec3(0.0, 0.0, 3.0),
        glm::vec3(0.0, 1.0, 0.0),
        camera::YAW,
        camera::PITCH,
    );

    state.wireframe = false;

    let mut lighting_shader = Shader::new(
        "assets/shaders/basic_lighting.vs",
        "assets/shaders/basic_lighting.fs",
    );
    let mut light_cube_shader = Shader::new(
        "assets/shaders/light_cube.vs",
        "assets/shaders/light_cube.fs",
    );

    #[rustfmt::skip]
    let vertices = [
       -0.5f32, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
    ];

    let mut cube_vao = 0;
    let mut vbo = 0;
    let mut light_vao = 0;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        gl::GenVertexArrays(1, &mut cube_vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(cube_vao);

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

        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
    }

    while !window.should_close() {
        glfw.poll_events();

        let current_frame: f32 = unsafe { glfwGetTime() as f32 };
        state.delta_time = current_frame - state.last_frame;
        state.last_frame = current_frame;

        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event, &mut state, &mut camera);
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            state.light_position.x = 1.0 + (glfwGetTime() as f32).sin() * 2.0;
            state.light_position.y = ((glfwGetTime() as f32).sin() / 2.0) * 1.0;

            lighting_shader.use_program();
            lighting_shader.set_vec3("objectColor", glm::vec3(1.0, 0.5, 0.31));
            lighting_shader.set_vec3("lightColor", glm::vec3(1.0, 1.0, 1.0));
            lighting_shader.set_vec3("lightPos", state.light_position);
            lighting_shader.set_vec3("viewPos", camera.position);

            let projection;

            // Note the glm rust port has these paramaters wrong lol
            projection = glm::perspective(
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                camera.zoom.to_radians(),
                0.1,
                100.0,
            );

            lighting_shader.set_mat4("projection", projection);

            let view = camera.get_view_matrix();
            lighting_shader.set_mat4("view", view);

            let mut model = glm::Mat4::identity();
            lighting_shader.set_mat4("model", model);

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            ///////

            light_cube_shader.use_program();
            light_cube_shader.set_mat4("projection", projection);
            light_cube_shader.set_mat4("view", view);

            model = glm::Mat4::identity();
            model = glm::translate(&model, &state.light_position);
            model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
            light_cube_shader.set_mat4("model", model);

            gl::BindVertexArray(light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl_last_error();
        }

        window.swap_buffers();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &cube_vao);
        gl::DeleteVertexArrays(1, &light_vao);
        gl::DeleteBuffers(1, &vbo);
        light_cube_shader.delete_program();
        lighting_shader.delete_program();
    }
}

// This would be the process input function in the C++ version
fn glfw_handle_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    state: &mut State,
    camera: &mut Camera,
) {
    use glfw::Action;
    use glfw::Key;

    unsafe {
        let key_state = glfwGetKey(window.window_ptr(), glfw::ffi::KEY_W);

        if key_state == glfw::ffi::PRESS {
            camera.process_keyboard(camera::CameraMovement::FORWARD, state.delta_time);
        }
    }

    match event {
        glfw::WindowEvent::CursorPos(x_position, y_position) => {
            if state.first_mouse {
                state.last_mouse_x = x_position as f32;
                state.last_mouse_y = y_position as f32;
                state.first_mouse = false;
            }

            let x_offset = (x_position as f32) - state.last_mouse_x;
            let y_offset = state.last_mouse_y - (y_position as f32);

            state.last_mouse_x = x_position as f32;
            state.last_mouse_y = y_position as f32;
            camera.process_mouse_movement(x_offset, y_offset, true);
        }
        glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
            camera.process_mouse_scroll(y_offset);
        }
        glfw::WindowEvent::Key(key, _scancode, action, _mods) => match (key, action) {
            (Key::Escape, Action::Press) => {
                window.set_should_close(true);
            }
            (Key::F, Action::Press) => {
                handle_wireframe(state);
            }
            (Key::W | Key::A | Key::D | Key::S, Action::Press) => {
                if key == glfw::Key::W {
                    //camera.process_keyboard(camera::CameraMovement::FORWARD, state.delta_time);
                }

                if key == glfw::Key::S {
                    camera.process_keyboard(camera::CameraMovement::BACKWARD, state.delta_time);
                }

                if key == glfw::Key::A {
                    camera.process_keyboard(camera::CameraMovement::LEFT, state.delta_time);
                }

                if key == glfw::Key::D {
                    camera.process_keyboard(camera::CameraMovement::RIGHT, state.delta_time);
                }
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

            match errno {
                gl::INVALID_OPERATION => {
                    panic!("OpenGL Invalid Operation Last Frame");
                }
                _ => {}
            }
        }
    }
}
