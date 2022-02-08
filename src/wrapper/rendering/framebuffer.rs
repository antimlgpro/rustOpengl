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

	pub fn gen_texture_buffer(&mut self, tex_buf: TextureBuffer) {
		unsafe {
			let mut buf: u32 = 0;

			gl::GenTextures(1, &mut buf);
			gl::BindTexture(gl::TEXTURE_2D, buf);
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				tex_buf.internal_format as i32,
				tex_buf.width as i32,
				tex_buf.height as i32,
				0,
				tex_buf.format,
				tex_buf.type_,
				std::ptr::null(),
			);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			gl::FramebufferTexture2D(gl::FRAMEBUFFER, tex_buf.attachment, gl::TEXTURE_2D, buf, 0);

			self.buffers.insert(tex_buf.name, buf);
			self.attachments.push(tex_buf.attachment);
		}
	}

	pub fn draw_buffers(&mut self) {
		unsafe {
			gl::DrawBuffers(self.attachments.len() as i32, self.attachments.as_ptr());
		}
	}

	pub fn add_attachments(&mut self, attachments: &mut Vec<GLenum>) {
		self.attachments.append(attachments);
	}

	pub fn get_buffer(&self, name: &str) -> &u32 {
		return self.buffers.get(name).unwrap();
	}
}
