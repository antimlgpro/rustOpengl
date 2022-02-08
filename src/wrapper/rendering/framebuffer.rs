use crate::wrapper::render::core::Texture;

use gl::types::*;
use std::collections::HashMap;

pub struct TextureBuffer {
	pub name: String,
	pub width: u32,
	pub height: u32,
	pub type_: GLenum,
	pub internal_format: GLenum,
	pub format: GLenum,
	pub attachment: GLenum,
}

pub struct FrameBuffer {
	pub fbo: u32,
	pub buffers: HashMap<String, u32>,
	pub attachments: Vec<GLenum>,
}

impl FrameBuffer {
	pub fn new() -> Self {
		let mut frame = FrameBuffer {
			fbo: 0,
			buffers: HashMap::default(),
			attachments: Vec::new(),
		};

		unsafe {
			let mut fbo: u32 = 0;
			gl::GenFramebuffers(1, &mut fbo);
			gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
			frame.fbo = fbo;
		}

		frame
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
		}
	}

	pub fn finish(&self) -> Result<(), &str> {
		unsafe {
			let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
			if status != gl::FRAMEBUFFER_COMPLETE {
				return Err("Framebuffer not complete!");
			}

			// bind frame buffer
			self.unbind();
			return Ok(());
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}
	}

	pub fn add_texture(&mut self, texture: Texture) {
		self.buffers.insert(texture.type_name, texture.id);
		self.attachments.push(gl::COLOR_ATTACHMENT0 + texture.index);
	}

	pub fn draw_buffers(&mut self) {
		unsafe {
			gl::DrawBuffers(self.attachments.len() as i32, self.attachments.as_ptr());
		}
	}

	pub fn get_buffer(&self, name: &str) -> &u32 {
		return self.buffers.get(name).unwrap();
	}
}
