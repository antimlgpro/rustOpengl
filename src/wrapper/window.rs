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

#[derive(Default)]
pub struct Time {
	pub last_frame: f64,
	pub delta_time: f32,
	pub frame_count: i32,
	pub fps: f32,

	last_time: f32,
	frames_per_second: i32,
}

impl Time {
	pub fn update(&mut self, time: f64) {
		self.delta_time = (time - self.last_frame) as f32;
		self.last_frame = time;
		self.frame_count += 1;

		self.frames_per_second += 1;
		if (time as f32 - self.last_time) > 1.0 {
			self.last_time = time as f32;
			self.fps = 1000.0 / self.frames_per_second as f32;
			self.frames_per_second = 0;
		}
	}
}

/*
impl Default for Time {
	fn default() -> Time {
		Time {
			time: 0.0,
			delta_time: 0.0,
			frame_count: 0,
			fps: 0.0,
			last_frame: 0.0,
		}
	}
}*/

pub struct Window {
	pub settings: WindowSettings,
	pub time: Time,

	internal_window: glfw::Window,
	glfw: glfw::Glfw,
	events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
	pub fn new(settings: WindowSettings) -> Window {
		let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		// Create a windowed mode window and its OpenGL context
		let (window, events) = glfw
			.create_window(
				settings.width,
				settings.height,
				settings.title.as_str(),
				glfw::WindowMode::Windowed,
			)
			.expect("Failed to create GLFW window.");

		// Provides highest available version
		glfw.window_hint(glfw::WindowHint::ContextVersion(1, 0));
		glfw.window_hint(glfw::WindowHint::OpenGlProfile(
			glfw::OpenGlProfileHint::Core,
		));

		Window {
			settings,
			glfw,
			events,
			internal_window: window,
			time: Time::default(),
		}
	}

	pub fn init(mut self) -> Window {
		self.internal_window.make_current();
		self.internal_window.set_key_polling(true);
		self.internal_window.set_framebuffer_size_polling(true);

		gl::load_with(|symbol| self.internal_window.get_proc_address(symbol) as *const _);

		return self;
	}

	pub fn gl_enable(&self, e: gl::types::GLenum) {
		unsafe {
			gl::Enable(e);
		}
	}

	pub fn get_frame(&self) -> Frame {
		Frame::new()
	}

	pub fn should_close(&self) -> bool {
		return self.internal_window.should_close();
	}

	pub fn get_aspect_ratio(&self) -> f32 {
		return self.settings.width as f32 / self.settings.height as f32;
	}

	pub fn pre_loop(&mut self) {
		self.time.update(self.glfw.get_time());
	}

	pub fn post_loop(&mut self) {
		self.internal_window.swap_buffers();
		self.glfw.poll_events();
	}

	pub fn get_time(&self) -> &Time {
		return &self.time;
	}

	// ----------- GLFW functions -----------
}
