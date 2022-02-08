use gl::types::*;

pub struct Frame {}

impl Frame {
	pub fn draw() -> Self {
		Frame {}
	}

	pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
		unsafe {
			gl::ClearColor(r, g, b, a);
		}
	}

	pub fn clear(&self, enmu: GLenum) {
		unsafe {
			gl::Clear(enmu);
		}
	}
}
