#![allow(dead_code)]
extern crate gl;
extern crate glfw;

use legion::*;
use nalgebra::{vector, Matrix4, Rotation3, Vector3};

mod components;
mod util;
mod wrapper;
use components::*;
use util::radians;
use wrapper::{Mesh, Shader, Vertex, Window, WindowSettings};

fn game_loop() {}

fn main() {
	let mut window = Window::new(WindowSettings::default()).init();

	window.gl_enable(gl::DEPTH_TEST);

	let vertices: [f32; 8 * 3] = [
		-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
		-1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
	];
	let indices = [
		0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2, 6,
		4, 5, 0, 0, 5, 1,
	];

	let mut vertices_: Vec<Vector3<f32>> = Vec::new();

	for i in (0..vertices.len()).step_by(3) {
		vertices_.push(Vector3::new(vertices[i], vertices[i + 1], vertices[i + 2]))
	}

	let ind: Vec<u32> = indices.to_vec();
	let mut vert: Vec<Vertex> = Vec::new();

	let normals = util::calculate_normals(&vertices_, &ind);

	for i in 0..vertices_.len() {
		vert.push(Vertex {
			position: vertices_[i],
			normal: normals[i],
		});
	}

	let mut world = legion::World::default();

	let shader = Shader::new("shaders/light.vs", "shaders/light.fs").unwrap();
	let light_shader = Shader::new("shaders/advanced.vs", "shaders/advanced.fs").unwrap();
	let mesh = Mesh::new(vert, ind).unwrap();

	let player = world.push((
		Transform {
			position: vector![0.0, 2.0, -3.0],
			rotation: Rotation3::from_euler_angles(radians(35.0), 0.0, 0.0),
			..Transform::default()
		},
		Camera {
			aspect_ratio: window.get_aspect_ratio(),
			..Camera::default()
		},
	));

	let mut model_transform = Transform {
		position: vector![0.0, 0.0, 0.0],
		..Transform::default()
	};

	let mut light_transform = Transform {
		position: vector![-3.0, 0.0, 3.0],
		..Transform::default()
	};

	while !window.should_close() {
		window.pre_loop();
		let frame = window.get_frame();
		frame.clear_color(0.1, 0.1, 0.1, 1.0);
		frame.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

		let time = window.get_time();
		let delta_time = time.delta_time;

		let mut query = <(&mut Transform, &mut Camera)>::query();
		for (transform, camera) in query.iter_mut(&mut world) {
			model_transform.rotate_euler(0.0, radians(90.0) * delta_time, 0.0);

			let mut light_pos = light_transform.position;
			light_pos.x = 1.0 + (time.last_frame as f32).sin() * 2.0;
			light_pos.y = (time.last_frame as f32 / 2.0).sin() * 1.0;
			light_transform.position = light_pos;

			transform.update_directions();
			camera.update(transform);
			let model = model_transform.get_matrix();

			shader.use_program();
			shader.set_vec3("object_color", 0.0, 0.49, 1.0);
			shader.set_vec3("light_color", 1.0, 1.0, 1.0);
			shader.set_vector3("light_pos", &light_transform.position);
			shader.set_vector3("view_pos", &transform.position);

			shader.set_mat4("proj", &camera.projection);
			shader.set_mat4("view", &camera.view);
			shader.set_mat4("model", &model);

			mesh.draw();

			let light_model = light_transform.get_matrix();

			light_shader.use_program();
			light_shader.set_mat4("proj", &camera.projection);
			light_shader.set_mat4("view", &camera.view);
			light_shader.set_mat4("model", &light_model);

			mesh.draw();
		}

		game_loop();
		window.post_loop();
	}
}
