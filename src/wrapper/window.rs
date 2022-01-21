extern crate gl;
extern crate glfw;
use self::glfw::Context;
use std::sync::mpsc::Receiver;

use super::frame::Frame;

/// Settings for window object
pub struct WindowSettings {
	pub width: u32,
	pub height: u32,
	pub title: String,
}

impl Default for WindowSettings {
	/// 800, 800
	fn default() -> WindowSettings {
		WindowSettings {
			width: 800,
			height: 800,
			title: "Window".to_string(),
		}
	}
}

pub struct Window {
	pub settings: WindowSettings,

	internal_window: glfw::Window,
	glfw: glfw::Glfw,
	events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
	pub fn new(settings: WindowSettings) -> Window {
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		// Create a windowed mode window and its OpenGL context
		let (window, events) = glfw
			.create_window(
				settings.width,
				settings.height,
				settings.title.as_str(),
				glfw::WindowMode::Windowed,
			)
			.expect("Failed to create GLFW window.");

		Window {
			settings,
			glfw,
			events,
			internal_window: window,
		}
	}

	pub fn init(mut self) -> Window {
		self.internal_window.make_current();
		self.internal_window.set_key_polling(true);
		self.internal_window.set_framebuffer_size_polling(true);

		gl::load_with(|symbol| self.internal_window.get_proc_address(symbol) as *const _);

		return self;
	}

	pub fn draw(&self) -> Frame {
		Frame::new()
	}

	pub fn should_close(&self) -> bool {
		return self.internal_window.should_close();
	}

	pub fn post_loop(&mut self) {
		self.internal_window.swap_buffers();
		self.glfw.poll_events();
	}
}
