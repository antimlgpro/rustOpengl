#![allow(dead_code)]
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use legion::*;
use nalgebra::{vector, Matrix4, Rotation3, Vector3};
use std::mem::size_of;

mod components;
mod util;
mod wrapper;
use components::*;
use util::radians;
use wrapper::{Mesh, Shader, Ubo, Vertex, Window, WindowSettings};

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

	let shader = Shader::new("shaders/ubo.vs", "shaders/ubo.fs").unwrap();
	let shader2 = Shader::new("shaders/advanced.vs", "shaders/advanced.fs").unwrap();
	let mesh = Mesh::new(vert, ind).unwrap();

	let player = world.push((
		Transform {
			position: vector![0.0, 3.0, -4.5],
			rotation: Rotation3::from_euler_angles(radians(20.0), 0.0, 0.0),
			..Transform::default()
		},
		Camera {
			aspect_ratio: window.get_aspect_ratio(),
			..Camera::default()
		},
	));

	let mut cube1_transform = Transform {
		position: vector![0.0, 0.0, 0.0],
		..Transform::default()
	};

	let light_transform = Transform {
		position: vector![0.0, 4.0, 0.0],
		..Transform::default()
	};

	let point = 0;
	Ubo::set_uniform_block(&shader, "Matrices", point);
	Ubo::set_uniform_block(&shader2, "Matrices", point);
	let ubo_matrices = match Ubo::create_buffer(point, 2 * size_of::<Matrix4<f32>>()) {
		Ok(e) => e,
		Err(e) => {
			panic!("Create_buffer: {}", e);
		}
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
			transform.update_directions();
			camera.update_view(transform);

			ubo_matrices.set_data_mat4(0, size_of::<Matrix4<f32>>(), camera.projection);
			ubo_matrices.set_data_mat4(
				size_of::<Matrix4<f32>>(),
				size_of::<Matrix4<f32>>(),
				camera.view,
			);

			cube1_transform.rotate_euler(0.0, radians(90.0) * delta_time, 0.0);
			let model = cube1_transform.get_matrix();

			shader.use_program();
			shader.set_vec3("object_color", 0.0, 0.49, 1.0);
			shader.set_vec3("light_color", 1.0, 1.0, 1.0);
			shader.set_vector3("light_pos", &light_transform.position);
			shader.set_vector3("view_pos", &vector!(0.0, 0.0, 0.0));
			shader.set_mat4("model", &model);
			mesh.draw();

			shader2.use_program();
			let mut model2 = light_transform.get_matrix();
			model2 = glm::scale(&model2, &vector!(0.5, 0.5, 0.5));

			shader2.set_vec3("object_color", 0.9, 0.9, 0.9);
			shader2.set_mat4("model", &model2);
			mesh.draw();
		}

		game_loop();
		window.post_loop();
	}
}
