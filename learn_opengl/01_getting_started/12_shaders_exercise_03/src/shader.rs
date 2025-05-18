use std::{ffi::CString, fs};

use gl;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Shader {
        let vertex_shader = Self::compile_shader(gl::VERTEX_SHADER, vertex_shader_path);
        let fragment_shader = Self::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_path);

        let shader = Self::compile_program(vertex_shader, fragment_shader);

        Shader { id: shader }
    }

    fn read_shader_from_file(file_path: &str) -> String {
        let raw_shader_source = fs::read_to_string(file_path);

        let shader_source = raw_shader_source.unwrap();

        return shader_source;
    }

    fn compile_shader(shader_type: gl::types::GLenum, shader_path: &str) -> u32 {
        let binding = Self::read_shader_from_file(&shader_path);
        let shader_source: &str = binding.as_str();

        let foo = CString::new(shader_source).unwrap();

        let compiled_shader = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(compiled_shader, 1, &foo.as_ptr(), std::ptr::null());

            gl::CompileShader(compiled_shader);

            let mut success = 0;
            gl::GetShaderiv(compiled_shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut log_len = 0_i32;
                let mut v: Vec<u8> = Vec::with_capacity(512);
                gl::GetShaderInfoLog(compiled_shader, 512, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!(
                    "ERROR::SHADER::{}::COMPILATION_FAILED: {}",
                    shader_type,
                    String::from_utf8_lossy(&v)
                );
            }

            return compiled_shader;
        }
    }

    fn compile_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
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

        return shader_program;
    }

    pub fn use_program(&mut self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn delete_program(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        unsafe {
            let location_name = CString::new(name.to_string()).unwrap();
            let location_name_ptr = location_name.as_ptr();
            let location = gl::GetUniformLocation(self.id, location_name_ptr);
            gl::Uniform1i(location, value as i32);
        }
    }

    pub fn set_int(&mut self, name: &str, value: i32) {
        unsafe {
            let location_name = CString::new(name.to_string()).unwrap();
            let location_name_ptr = location_name.as_ptr();
            let location = gl::GetUniformLocation(self.id, location_name_ptr);
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        unsafe {
            let location_name = CString::new(name.to_string()).unwrap();
            let location_name_ptr = location_name.as_ptr();
            let location = gl::GetUniformLocation(self.id, location_name_ptr);
            gl::Uniform1f(location, value);
        }
    }
}
