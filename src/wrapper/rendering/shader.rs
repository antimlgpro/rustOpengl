extern crate gl;
use self::gl::types::*;
use crate::util::{create_whitespace_cstring_with_len, load_to_string, to_cstring};
use nalgebra::{Matrix4, Vector3};
use std::{cmp::max, ffi::CString, fmt, fmt::Display, ptr, str};

#[derive(Debug, Clone)]
pub enum ShaderError {
	Compile(String),
	Linking(String),
}

impl Display for ShaderError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ShaderError::Compile(e) => write!(f, "{}", e),
			ShaderError::Linking(e) => write!(f, "{}", e),
		}
	}
}

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

impl Shader {
	/// Opens and creates a shader program
	///
	/// * `v_src_path` - String path to vertex shader
	/// * `f_src_path` - String path to fragment shader
	pub fn new(v_src_path: &str, f_src_path: &str) -> Result<Shader, ShaderError> {
		let mut shader = Shader { id: 0 };

		let vertex_src = to_cstring(load_to_string(v_src_path).unwrap()).unwrap();
		let fragment_src = to_cstring(load_to_string(f_src_path).unwrap()).unwrap();

		// TODO check shader compile errors
		unsafe {
			let vertex = gl::CreateShader(gl::VERTEX_SHADER);
			gl::ShaderSource(vertex, 1, &vertex_src.as_ptr(), ptr::null());
			gl::CompileShader(vertex);
			match shader.check_shader_errors(vertex) {
				Some(e) => {
					println!("Vertex: {}", e)
				}
				None => {}
			}

			let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
			gl::ShaderSource(fragment, 1, &fragment_src.as_ptr(), ptr::null());
			gl::CompileShader(fragment);
			match shader.check_shader_errors(fragment) {
				Some(e) => {
					println!("Fragment: {}", e)
				}
				None => {}
			}

			let id = gl::CreateProgram();
			gl::AttachShader(id, vertex);
			gl::AttachShader(id, fragment);
			gl::LinkProgram(id);
			match shader.check_program_errors(id) {
				Some(e) => {
					println!("Program: {}", e);
				}
				None => {}
			}
			gl::DeleteShader(vertex);
			gl::DeleteShader(fragment);

			shader.id = id;
		}

		Ok(shader)
	}

	pub fn use_program(&self) {
		unsafe {
			gl::UseProgram(self.id);
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

	fn check_compiler_errors(&self, shader: u32, type_: ShaderType) -> Option<ShaderError> {
		unsafe {
			let mut shader_iv = gl::FALSE as GLint;
			let mut program_iv = gl::FALSE as GLint;

			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut shader_iv);
			gl::GetProgramiv(shader, gl::LINK_STATUS, &mut program_iv);

			let len = max(shader_iv, program_iv);
			let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
			// fill it with len spaces
			buffer.extend([b' '].iter().cycle().take(len as usize));
			// convert buffer to CString
			let error: CString = CString::from_vec_unchecked(buffer);

			match type_ {
				ShaderType::VERTEX => {
					if shader_iv != gl::TRUE as GLint {
						gl::GetShaderInfoLog(
							shader,
							len,
							ptr::null_mut(),
							error.as_ptr() as *mut GLchar,
						);
						let err = format!(
							"Vertex shader compilation error: {}",
							error.to_string_lossy().into_owned()
						);

						return Some(ShaderError::Compile(err));
					}
				}
				ShaderType::FRAGMENT => {
					if shader_iv != gl::TRUE as GLint {
						gl::GetShaderInfoLog(
							shader,
							len,
							ptr::null_mut(),
							error.as_ptr() as *mut GLchar,
						);
						let err = format!(
							"Fragment shader compilation error: {}",
							error.to_string_lossy().into_owned()
						);
						return Some(ShaderError::Compile(err));
					}
				}
				ShaderType::PROGRAM => {
					if program_iv != gl::TRUE as GLint {
						gl::GetProgramInfoLog(
							shader,
							len,
							ptr::null_mut(),
							error.as_ptr() as *mut GLchar,
						);
						let err = format!(
							"Program linking error: '{}'",
							error.to_string_lossy().into_owned()
						);

						return Some(ShaderError::Linking(err));
					}
				}
			}

			None
		}
	}
}
