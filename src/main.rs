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
use wrapper::{Mesh, Shader, Ubo, UniformManager, Vertex, Window, WindowSettings};

fn render_model(tf: &Transform, rend: &Renderable, unif_man: &mut UniformManager) {
	let mesh = &rend.mesh;
	let shader = &rend.material.shader;
	let model = tf.get_matrix();

	shader.use_program();
	shader.set_mat4("model", &model);

	rend.material.set_uniforms_vec3(unif_man);
	rend.material.set_uniforms_mat4(unif_man);

	mesh.draw();
}

fn gen_cube() -> (Vec<Vertex>, Vec<u32>) {
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

	return (vert, ind);
}

fn main() {
	let mut window = Window::new(WindowSettings::default()).default_setup();
	let mut world = legion::World::default();
	let mut uniform_man = UniformManager::new();

	let shader = Shader::new("shaders/ubo.vs", "shaders/ubo.fs").unwrap();

	uniform_man.add_uniform("object_color", vector!(0.0, 0.49, 0.1));
	uniform_man.add_uniform("light_color", vector!(1.0, 1.0, 1.0));
	uniform_man.add_uniform("light_pos", vector!(0.0, 5.0, 0.0));
	uniform_man.add_uniform("view_pos", vector!(0.0, 0.0, 0.0));

	let (vert, ind) = gen_cube();
	let mesh = Mesh::new(vert, ind).unwrap();

	let point = 0;
	Ubo::set_uniform_block(&shader, "Matrices", point);
	//Ubo::set_uniform_block(&shader2, "Matrices", point);
	let ubo_matrices = match Ubo::create_buffer(point, 2 * size_of::<Matrix4<f32>>()) {
		Ok(e) => e,
		Err(e) => {
			panic!("Create_buffer: {}", e);
		}
	};

	let test_mat = Material::new(
		shader,
		vec!["object_color", "light_color", "light_pos", "view_pos"],
	);

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

	let test = world.push((
		Transform {
			position: vector![0.0, 0.0, 0.0],
			..Transform::default()
		},
		Renderable {
			mesh: mesh,
			material: test_mat,
		},
	));

	while !window.should_close() {
		window.pre_loop();
		let frame = window.get_frame();
		frame.clear_color(0.1, 0.1, 0.1, 1.0);
		frame.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

		let time = window.get_time();
		let delta_time = time.delta_time;

		let mut query = <(&mut Transform, &mut Camera)>::query();
		for (tf, camera) in query.iter_mut(&mut world) {
			tf.update_directions();
			camera.update_view(tf);

			ubo_matrices.set_data_mat4(0, size_of::<Matrix4<f32>>(), camera.projection);
			ubo_matrices.set_data_mat4(
				size_of::<Matrix4<f32>>(),
				size_of::<Matrix4<f32>>(),
				camera.view,
			);

			uniform_man.set_uniform("view_pos", tf.position);
		}

		let mut query = <(&mut Transform, &mut Renderable)>::query();
		for (transform, renderable) in query.iter_mut(&mut world) {
			render_model(transform, renderable, &mut uniform_man);
		}

		window.post_loop();
	}
}
