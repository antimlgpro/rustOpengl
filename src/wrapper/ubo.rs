use super::{error::*, shader::Shader};
use gl::types::*;
use nalgebra::{Matrix4, Vector3};
use std::{ffi::CString, os::raw::c_void, ptr};

pub struct Ubo {
	pub id: u32,
}

impl Ubo {
	pub fn create_buffer(bind_point: GLuint, size: usize) -> Result<Ubo, GLError> {
		unsafe {
			let mut id = 0;
			gl::GenBuffers(1, &mut id);
			gl::BindBuffer(gl::UNIFORM_BUFFER, id);
			gl::BufferData(
				gl::UNIFORM_BUFFER,
				size as isize,
				ptr::null(),
				gl::STATIC_DRAW,
			);
			gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
			gl::BindBufferRange(gl::UNIFORM_BUFFER, bind_point, id, 0, size as isize);

			match get_error() {
				Some(e) => return Err(e),
				None => return Ok(Ubo { id }),
			};
		};
	}

	pub fn set_uniform_block(shader: &Shader, name: &str, bind_point: GLuint) -> GLuint {
		unsafe {
			let _name = &CString::new(name).expect("Unable to convert string to CString");
			let idx = gl::GetUniformBlockIndex(shader.id, _name.as_ptr());
			gl::UniformBlockBinding(shader.id, idx, bind_point);

			match get_error() {
				Some(e) => {
					println!("uniform: {}", e);
				}
				None => {}
			};

			return idx;
		}
	}

	pub fn set_data_mat4(&self, offset: usize, size: usize, data: Matrix4<f32>) {
		unsafe {
			gl::BindBuffer(gl::UNIFORM_BUFFER, self.id);
			gl::BufferSubData(
				gl::UNIFORM_BUFFER,
				offset as GLintptr,
				size as GLsizeiptr,
				data.as_ptr() as *const c_void,
			);
			gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
		}
	}

	pub fn set_data_vec3(&self, offset: usize, size: usize, data: Vector3<f32>) {
		unsafe {
			gl::BindBuffer(gl::UNIFORM_BUFFER, self.id);
			gl::BufferSubData(
				gl::UNIFORM_BUFFER,
				offset as GLintptr,
				size as GLsizeiptr,
				data.as_ptr() as *const c_void,
			);
			gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
		}
	}
}
