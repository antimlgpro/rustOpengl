#![allow(non_upper_case_globals)]
use std::{mem::size_of, os::raw::c_void};

const quad_vertices: [f32; 24] = [
	-1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, -1.0,
	1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
];

const cube_vertices: [f32; 288] = [
	-1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, 1.0,
	-1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, -1.0, -1.0,
	-1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, -1.0, -1.0, 1.0,
	0.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
	1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
	-1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 1.0,
	-1.0, -1.0, 0.0, 0.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, -1.0, -1.0, -1.0,
	-1.0, 0.0, 0.0, 0.0, 1.0, -1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0, -1.0, 0.0,
	0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0,
	1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0,
	1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -1.0, -1.0,
	0.0, -1.0, 0.0, 0.0, 1.0, 1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0, -1.0,
	0.0, 1.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0,
	0.0, -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
	1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
	1.0, 0.0, 1.0, 0.0, -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
	0.0, 0.0,
];

pub trait Primitive {
	fn new() -> Self;
	fn draw(&self);
}

pub struct Quad {
	vao: u32,
	vbo: u32,
}

pub struct Cube {
	vao: u32,
	vbo: u32,
}

impl Primitive for Quad {
	fn new() -> Quad {
		unsafe {
			let mut vao: u32 = 0;
			let mut vbo: u32 = 0;
			gl::GenVertexArrays(1, &mut vao);
			gl::GenBuffers(1, &mut vbo);
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(size_of::<f32>() * 24) as isize,
				quad_vertices.as_ptr() as *const c_void,
				gl::STATIC_DRAW,
			);
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				2,
				gl::FLOAT,
				gl::FALSE,
				(size_of::<f32>() * 4) as i32,
				0 as *const c_void,
			);
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE,
				(size_of::<f32>() * 4) as i32,
				(size_of::<f32>() * 2) as *const c_void,
			);

			Quad { vao, vbo }
		}
	}

	fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 6);
			gl::BindVertexArray(0);
		}
	}
}

impl Primitive for Cube {
	fn new() -> Cube {
		unsafe {
			let mut vao: u32 = 0;
			let mut vbo: u32 = 0;
			gl::GenVertexArrays(1, &mut vao);
			gl::GenBuffers(1, &mut vbo);
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(size_of::<f32>() * 24) as isize,
				cube_vertices.as_ptr() as *const c_void,
				gl::STATIC_DRAW,
			);
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				2,
				gl::FLOAT,
				gl::FALSE,
				(size_of::<f32>() * 8) as i32,
				0 as *const c_void,
			);
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				2,
				gl::FLOAT,
				gl::FALSE,
				(size_of::<f32>() * 8) as i32,
				(size_of::<f32>() * 3) as *const c_void,
			);
			gl::EnableVertexAttribArray(2);
			gl::VertexAttribPointer(
				2,
				2,
				gl::FLOAT,
				gl::FALSE,
				(size_of::<f32>() * 8) as i32,
				(size_of::<f32>() * 6) as *const c_void,
			);

			Cube { vao, vbo }
		}
	}

	fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 36);
			gl::BindVertexArray(0);
		}
	}
}
