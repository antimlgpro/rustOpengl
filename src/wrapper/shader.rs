extern crate gl;
use self::gl::types::*;
use crate::util::{load_to_string, to_cstring};
use nalgebra::{Matrix4, Vector3};
use std::{cmp::max, error::Error, ffi::CString, fmt, fmt::Display, ptr, str};

#[derive(Debug, Clone)]
enum ShaderError {
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

pub struct Shader {
	id: u32,
}

impl Shader {
	/// Opens and creates a shader program
	///
	/// * `v_src_path` - String path to vertex shader
	/// * `f_src_path` - String path to fragment shader
	pub fn new(v_src_path: &str, f_src_path: &str) -> Result<Shader, Box<dyn Error>> {
		let mut shader = Shader { id: 0 };

		let vertex_src = to_cstring(load_to_string(v_src_path).unwrap()).unwrap();
		let fragment_src = to_cstring(load_to_string(f_src_path).unwrap()).unwrap();

		// TODO check shader compile errors
		unsafe {
			let vertex = gl::CreateShader(gl::VERTEX_SHADER);
			gl::ShaderSource(vertex, 1, &vertex_src.as_ptr(), ptr::null());
			gl::CompileShader(vertex);
			shader
				.check_compiler_errors(vertex, ShaderType::VERTEX)
				.unwrap_or_else(|e| {
					eprintln!("{}", e);
				});

			let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
			gl::ShaderSource(fragment, 1, &fragment_src.as_ptr(), ptr::null());
			gl::CompileShader(fragment);
			shader
				.check_compiler_errors(vertex, ShaderType::FRAGMENT)
				.unwrap_or_else(|e| {
					eprintln!("{}", e);
				});

			let id = gl::CreateProgram();
			gl::AttachShader(id, vertex);
			gl::AttachShader(id, fragment);
			gl::LinkProgram(id);
			shader
				.check_compiler_errors(id, ShaderType::PROGRAM)
				.unwrap_or_else(|e| {
					eprintln!("{}", e);
				});

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

	fn check_compiler_errors(&self, shader: u32, type_: ShaderType) -> Result<(), ShaderError> {
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

						return Err(ShaderError::Compile(err));
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
						return Err(ShaderError::Compile(err));
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
							"Program linking error: '{0}'",
							error.to_string_lossy().into_owned()
						);

						return Err(ShaderError::Linking(err));
					}
				}
			}

			Ok(())
		}
	}
}
