#![allow(dead_code)]
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use legion::*;
use nalgebra::{vector, Matrix4, Rotation3};
use std::mem::size_of;

mod components;
mod util;
mod wrapper;
use components::*;
use util::radians;
use wrapper::{
	error_callback, Loader, Material, Shader, Ubo, UniformManager, Window, WindowSettings,
};

#[system(for_each)]
fn render_model(tf: &Transform, rend: &Renderable, #[resource] unif_man: &mut UniformManager) {
	let mesh = &rend.mesh;
	let shader = &rend.material.shader;
	let model = tf.get_matrix();

	shader.use_program();
	shader.set_mat4("model", &model);
	rend.material.set_all(unif_man);

	mesh.draw();
}

#[system(for_each)]
fn update_camera(
	tf: &mut Transform,
	cam: &mut Camera,
	#[resource] unif_man: &mut UniformManager,
	#[resource] ubo_mat: &mut Ubo,
) {
	tf.update_directions();
	cam.update_view(tf);

	ubo_mat.set_data_mat4(0, size_of::<Matrix4<f32>>(), cam.projection);
	ubo_mat.set_data_mat4(
		size_of::<Matrix4<f32>>(),
		size_of::<Matrix4<f32>>(),
		cam.view,
	);

	unif_man.set_uniform("view_pos", tf.position);
}

fn main() {
	let mut window = Window::new(WindowSettings::default()).default_setup();
	window.debug_message_callback(Some(error_callback));

	let mut world = legion::World::default();
	let mut resources = Resources::default();
	let mut schedule = Schedule::builder()
		.add_thread_local(update_camera_system())
		.add_thread_local(render_model_system())
		.build();

	let shader = Shader::new("shaders/ubo.vs", "shaders/ubo.fs").unwrap();
	let mut uniform_man = UniformManager::new();
	uniform_man.add_uniform("light_color", vector!(1.0, 1.0, 1.0));
	uniform_man.add_uniform("light_pos", vector!(0.0, 5.0, 0.0));
	uniform_man.add_uniform("view_pos", vector!(0.0, 0.0, 0.0));

	let mut loaded = match Loader::load("models/teapot.obj") {
		Ok(m) => m,
		Err(e) => {
			panic!("Loader: {}", e);
		}
	};
	let mesh = loaded.remove(0);

	let point = 0;
	Ubo::set_uniform_block(&shader, "Matrices", point);
	let ubo_matrices = match Ubo::create_buffer(point, 2 * size_of::<Matrix4<f32>>()) {
		Ok(e) => e,
		Err(e) => {
			panic!("Create_buffer: {}", e);
		}
	};

	resources.insert(uniform_man);
	resources.insert(ubo_matrices);

	let test_mat = Material::new(
		shader,
		vec![("object_color", vector!(0.0, 0.49, 0.1))],
		vec!["light_color", "light_pos", "view_pos"],
	);

	let _player = world.push((
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

	let cube = world.push((
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

		if let Some(mut entry) = world.entry(cube) {
			let tf = entry.get_component_mut::<Transform>().unwrap();
			tf.rotate_euler(0.0, radians(90.0) * delta_time, 0.0);
		}

		schedule.execute(&mut world, &mut resources);

		window.post_loop();
	}
}
