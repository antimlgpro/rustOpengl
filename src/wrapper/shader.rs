extern crate gl;
use crate::util::{load_to_string, to_cstring};
use std::error::Error;
use std::ptr;

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

			let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
			gl::ShaderSource(fragment, 1, &fragment_src.as_ptr(), ptr::null());
			gl::CompileShader(fragment);

			let id = gl::CreateProgram();
			gl::AttachShader(id, vertex);
			gl::AttachShader(id, fragment);
			gl::LinkProgram(id);

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
}
