extern crate gl;
extern crate glfw;

mod wrapper;
use wrapper::{Window, WindowSettings};

fn game_loop() {}

fn main() {
	let mut window = Window::new(WindowSettings::default()).init();

	while !window.should_close() {
		let frame = window.draw();
		frame.clear_color(0.1, 1.0, 0.5, 1.0);
		frame.clear(gl::COLOR_BUFFER_BIT);

		game_loop();

		window.post_loop();
	}
}
