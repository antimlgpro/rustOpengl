use crate::util::{create_whitespace_cstring_with_len, load_to_string, to_cstring};
use crate::wrapper::error::ShaderError;
use nalgebra::{Matrix4, Vector3};
use std::{ffi::CString, ptr, str};

enum ShaderType {
	VERTEX,
	FRAGMENT,
	PROGRAM,
}

#[derive(Copy)]
pub struct Shader {
	pub id: u32,
}

impl Clone for Shader {
	fn clone(&self) -> Shader {
		*self
	}
}

// TODO: Add posibility to make other types of shaders
impl Shader {
	/// Creates new shader from file paths
	/// Uses vertex shader and fragment shader
	pub fn new(v_src_path: &str, f_src_path: &str) -> Result<Shader, ShaderError> {
		let mut shader = Shader { id: 0 };

		let vertex_src = to_cstring(load_to_string(v_src_path).unwrap()).unwrap();
		let fragment_src = to_cstring(load_to_string(f_src_path).unwrap()).unwrap();

		unsafe {
			let vertex = gl::CreateShader(gl::VERTEX_SHADER);
			gl::ShaderSource(vertex, 1, &vertex_src.as_ptr(), ptr::null());
			gl::CompileShader(vertex);
			match shader.check_shader_errors(vertex) {
				Some(e) => {
					//println!("Vertex: {}", e);
					return Err(e as ShaderError);
				}
				None => {}
			}

			let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
			gl::ShaderSource(fragment, 1, &fragment_src.as_ptr(), ptr::null());
			gl::CompileShader(fragment);
			match shader.check_shader_errors(fragment) {
				Some(e) => {
					//println!("Fragment: {}", e);
					return Err(e as ShaderError);
				}
				None => {}
			}

			let id = gl::CreateProgram();
			gl::AttachShader(id, vertex);
			gl::AttachShader(id, fragment);
			gl::LinkProgram(id);
			match shader.check_program_errors(id) {
				Some(e) => {
					//println!("Program: {}", e);
					return Err(e as ShaderError);
				}
				None => {}
			}
			gl::DeleteShader(vertex);
			gl::DeleteShader(fragment);

			shader.id = id;
		}

		Ok(shader)
	}

	/// Activates shader program
	pub fn use_program(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
	}

	fn check_shader_errors(&self, id: u32) -> Option<String> {
		unsafe {
			let mut success: gl::types::GLint = 1;
			gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

			if success == 0 {
				let mut len: gl::types::GLint = 0;
				gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

				let error = create_whitespace_cstring_with_len(len as usize);

				gl::GetShaderInfoLog(
					id,
					len,
					std::ptr::null_mut(),
					error.as_ptr() as *mut gl::types::GLchar,
				);

				return Some(error.to_string_lossy().into_owned());
			}
		}

		None
	}

	fn check_program_errors(&self, id: u32) -> Option<String> {
		unsafe {
			let mut success: gl::types::GLint = 1;
			gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

			if success == 0 {
				let mut len: gl::types::GLint = 0;
				gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

				let error = create_whitespace_cstring_with_len(len as usize);

				gl::GetProgramInfoLog(
					id,
					len,
					std::ptr::null_mut(),
					error.as_ptr() as *mut gl::types::GLchar,
				);

				return Some(error.to_string_lossy().into_owned());
			}
		}
		return None;
	}
}

/// Uniform setters
impl Shader {
	pub fn set_int(self, name: &str, val: i32) {
		unsafe {
			let _name = &CString::new(name).expect("Unable to convert string to CString");
			gl::Uniform1i(gl::GetUniformLocation(self.id, _name.as_ptr()), val);
		}
	}

	pub fn set_float(self, name: &str, val: f32) {
		unsafe {
			let _name = &CString::new(name).expect("Unable to convert string to CString");
			gl::Uniform1f(gl::GetUniformLocation(self.id, _name.as_ptr()), val);
		}
	}

	pub fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
		unsafe {
			let _name = &CString::new(name).expect("Unable to convert string to CString");
			gl::Uniform3f(gl::GetUniformLocation(self.id, _name.as_ptr()), x, y, z);
		}
	}

	pub fn set_vector3(&self, name: &str, value: &Vector3<f32>) {
		unsafe {
			let _name = &CString::new(name).expect("Unable to convert string to CString");
			gl::Uniform3fv(
				gl::GetUniformLocation(self.id, _name.as_ptr()),
				1,
				value.as_ptr(),
			);
		}
	}

	pub fn set_mat4(&self, name: &str, mat: &Matrix4<f32>) {
		unsafe {
			let _name = &CString::new(name).expect("Unable to convert string to CString");
			gl::UniformMatrix4fv(
				gl::GetUniformLocation(self.id, _name.as_ptr()),
				1,
				gl::FALSE,
				mat.as_ptr(),
			);
		}
	}
}
