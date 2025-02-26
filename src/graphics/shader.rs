use gl::types::*;
use std::ffi::CString;
use std::ptr;
use nalgebra_glm as glm;

pub struct Shader {
    program: GLuint,
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Self {
        let vertex_shader = Self::compile_shader(vertex_source, gl::VERTEX_SHADER);
        let fragment_shader = Self::compile_shader(fragment_source, gl::FRAGMENT_SHADER);
        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // 링크 에러 체크
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Shader linking failed: {}", error.to_string_lossy());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            program
        };

        Self { program }
    }

    fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Shader compilation failed: {}", error.to_string_lossy());
            }
        }
        shader
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.program, c_name.as_ptr()), value);
        }
    }

    pub fn set_vec4(&self, name: &str, value: &[f32; 4]) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            gl::Uniform4fv(
                gl::GetUniformLocation(self.program, c_name.as_ptr()),
                1,
                value.as_ptr()
            );
        }
    }

    pub fn set_mat4(&self, name: &str, value: &glm::Mat4) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.program, c_name.as_ptr()),
                1,
                gl::FALSE,
                value.as_ptr()
            );
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let buffer: Vec<u8> = vec![b' '; len];
    CString::new(buffer).unwrap()
}
