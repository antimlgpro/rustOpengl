pub struct RenderBuffer {
	rbo: u32,
}

impl RenderBuffer {
	pub fn new(width: u32, height: u32) -> Self {
		let mut buf = RenderBuffer { rbo: 0 };

		unsafe {
			let mut rbo: u32 = 0;
			gl::GenRenderbuffers(1, &mut rbo);
			gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
			gl::RenderbufferStorage(
				gl::RENDERBUFFER,
				gl::DEPTH_COMPONENT,
				width as i32,
				height as i32,
			);

			buf.rbo = rbo;
		}

		buf
	}

	pub fn framebuffer_renderbuffer(&self) {
		unsafe {
			gl::FramebufferRenderbuffer(
				gl::FRAMEBUFFER,
				gl::DEPTH_ATTACHMENT,
				gl::RENDERBUFFER,
				self.rbo,
			);
		}
	}
}
