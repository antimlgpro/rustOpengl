#![allow(dead_code)]
extern crate nalgebra_glm as glm;

use legion::*;
use nalgebra::{vector, Matrix4, Rotation3};
use rand::Rng;
use std::mem::size_of;

mod components;
mod util;
mod wrapper;
use components::*;
use util::radians;

use wrapper::{
	error::error_callback,
	render::{
		buffers::*,
		core::shader::Shader,
		core::*,
		primitive::{Primitive, Quad},
	},
	window::{Window, WindowSettings},
};

#[system(for_each)]
fn render_model(tf: &mut Transform, rend: &Renderable) {
	let mesh = &rend.mesh;
	let shader = &rend.material.shader;
	let model = tf.get_matrix();

	tf.rotate_euler(0.0, radians(1.0), 0.0);

	shader.use_program();
	shader.set_mat4("model", &model);
	shader.set_mat4(
		"normal_mat",
		&model.try_inverse().expect("Could not inverse?").transpose(),
	);

	// Sets material properties to shader
	rend.material.use_material();

	mesh.draw(&shader);
}

#[system(for_each)]
fn update_camera(
	tf: &mut Transform,
	cam: &mut Camera,
	//#[resource] unif_man: &mut UniformManager,
	#[resource] ubo_mat: &mut UniformBuffer,
) {
	tf.update_directions();
	cam.update_view(tf);

	ubo_mat.set_data_mat4(0, size_of::<Matrix4<f32>>(), cam.projection);
	ubo_mat.set_data_mat4(
		size_of::<Matrix4<f32>>(),
		size_of::<Matrix4<f32>>(),
		cam.view,
	);

	//unif_man.set_uniform("view_pos", tf.position);
}

fn main() {
	// Creates window
	let mut window = Window::new(WindowSettings::default()).default_setup();
	window.debug_message_callback(Some(error_callback));

	// Create world and schedule for ECS
	let mut world = legion::World::default();
	let mut resources = Resources::default();
	let mut render_schedule = Schedule::builder()
		.add_thread_local(update_camera_system())
		.add_thread_local(render_model_system())
		.build();

	let geometry_pass = Shader::new("shaders/pbr/geometry.vs", "shaders/pbr/geometry.fs").unwrap();
	let light_pass = Shader::new("shaders/pbr/light.vs", "shaders/pbr/light.fs").unwrap();
	//let cube_pass = Shader::new("shaders/advanced.vs", "shaders/advanced.fs").unwrap();

	let point = 0;
	UniformBuffer::set_uniform_block(&geometry_pass, "Matrices", point);
	let ubo_matrices = match UniformBuffer::create_buffer(point, 2 * size_of::<Matrix4<f32>>()) {
		Ok(e) => e,
		Err(e) => {
			panic!("Create_buffer: {}", e);
		}
	};
	resources.insert(ubo_matrices);

	let textur = Texture::from_file("texture1", "_textures/blank.png");

	let mut loaded = match Loader::load("models/teapot.obj") {
		Ok(m) => m,
		Err(e) => {
			panic!("Loader: {}", e);
		}
	};
	let mut mesh = loaded.remove(0);
	mesh.textures.clear();
	mesh.textures.push(textur);

	let cube_material = Material {
		shader: geometry_pass,
		albedo: vector!(199.0, 199.0, 199.0),
		metallic: 1.0,
		roughness: 1.0,
		ao: 1.0,
	};

	let g_buffer = {
		let (screen_width, screen_height) = (window.settings.width, window.settings.height);

		let tex_opt1 = TextureOptions {
			width: screen_width,
			height: screen_height,
			internal_format: gl::RGBA16F,
			format: gl::RGBA,
			type_: gl::FLOAT,
		};

		/*let tex_opt2 = TextureOptions {
			width: screen_width,
			height: screen_height,
			internal_format: gl::RGBA16F,
			format: gl::RGBA,
			type_: gl::UNSIGNED_BYTE,
		};*/

		let mut g_buffer = FrameBuffer::new();

		// Create framebuffer textures from options
		let g_position = Texture::for_framebuffer("position", 0, &tex_opt1);
		let g_normal = Texture::for_framebuffer("normal", 1, &tex_opt1);
		let g_albedo = Texture::for_framebuffer("albedo", 2, &tex_opt1);
		let g_material = Texture::for_framebuffer("material", 3, &tex_opt1);

		// Add textures to framebuffer
		g_buffer.add_texture(g_position);
		g_buffer.add_texture(g_normal);
		g_buffer.add_texture(g_albedo);
		g_buffer.add_texture(g_material);
		g_buffer.draw_buffers();

		// Create depth renderbuffer
		RenderBuffer::new(screen_width, screen_height);

		// Complete framebuffer and check for errors.
		g_buffer.finish().unwrap();
		g_buffer
	};

	light_pass.use_program();
	light_pass.set_int("g_position", 0);
	light_pass.set_int("g_normal", 1);
	light_pass.set_int("g_albedo", 2);
	light_pass.set_int("g_material", 3);

	let quad = Quad::new();

	let player_position = vector![0.0, 2.5, -2.5];
	let _player = world.push((
		Transform {
			position: player_position,
			rotation: Rotation3::from_euler_angles(radians(45.0), 0.0, 0.0),
			..Transform::default()
		},
		Camera {
			aspect_ratio: window.get_aspect_ratio(),
			..Camera::default()
		},
	));

	world.push((
		Transform {
			position: vector![0.0, 0.0, 0.0],
			scale: vector![0.7, 0.7, 0.7],
			..Transform::default()
		},
		Renderable {
			material: cube_material.clone(),
			mesh: mesh.clone(),
		},
	));

	/*
	// Creates cube array
	let space = 4;
	for x in 0..3 {
		for y in 0..3 {
			world.push((
				Transform {
					position: vector![(x * space) as f32, 0.0, (y * space) as f32],
					..Transform::default()
				},
				Renderable {
					material: cube_material.clone(),
					mesh: mesh.clone(),
				},
			));
		}
	}
	*/

	// Creates lights
	let mut rng = rand::thread_rng();
	for _ in 0..1 {
		world.push((
			Transform {
				position: vector![
					//rng.gen_range(-5.0..5.0),
					//rng.gen_range(-3.0..3.0),
					//rng.gen_range(-5.0..5.0)
					3.0, 3.0, -2.0
				],
				scale: vector![0.2, 0.2, 0.2],
				..Transform::default()
			},
			Light {
				color: vector![
					rng.gen_range(0.0..255.0) / 255.0,
					rng.gen_range(0.0..255.0) / 255.0,
					rng.gen_range(0.0..255.0) / 255.0
				],
				..Light::default()
			},
		));
	}

	while !window.should_close() {
		window.pre_loop();
		let frame = window.get_frame();

		let time = window.get_time();
		let delta_time = time.delta_time;

		// --------------
		// 1. Geometry pass

		// Bind frame buffer for deferred rendering
		// Binding framebuffer makes OpenGL render to buffer instead of window
		g_buffer.bind();
		frame.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

		render_schedule.execute(&mut world, &mut resources);

		/*
		let mut query = <(&mut Transform, &Light)>::query();
		for (tf, light) in query.iter_mut(&mut world) {
			let mesh = &mesh;
			let shader = &cube_pass;
			let model = tf.get_matrix();

			shader.use_program();
			shader.set_mat4("model", &model);
			shader.set_vector3("albedo", &light.color);

			mesh.draw(&shader);
		}
		*/

		// --------------
		// 2. Lighting pass

		// Enable blending
		unsafe {
			//gl::Enable(gl::BLEND);
			//gl::BlendFunc(gl::ONE, gl::ONE);
			gl::DepthFunc(gl::LEQUAL);
		}

		// Unbind gbuffer to render to default framebuffer.
		g_buffer.unbind();
		frame.clear(gl::COLOR_BUFFER_BIT); // | gl::DEPTH_BUFFER_BIT

		// Use lighting shader
		light_pass.use_program();
		g_buffer.activate_buffers(); // Set framebuffer textures

		// Loop over every light in scene
		let mut query = <(&mut Transform, &Light)>::query();
		for (tf, light) in query.iter_mut(&mut world) {
			light_pass.set_vector3("light_pos", &tf.position);
			light_pass.set_vector3("light_color", &light.color);

			// FIXME: figure out way to get player position
			// Player position is hardcoded for now.
			light_pass.set_vector3("camera_pos", &player_position);

			quad.draw();
		}

		// Disable blending
		unsafe {
			//gl::Disable(gl::BLEND);
			//gl::DepthFunc(gl::LESS);
			gl::DepthFunc(gl::LEQUAL);
		}

		// --------------
		// 2.5 copy content of geometry's depth buffer to default framebuffer's depth buffer
		unsafe {
			gl::BindFramebuffer(gl::READ_FRAMEBUFFER, g_buffer.fbo);
			gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0); // write to default framebuffer

			gl::BlitFramebuffer(
				0,
				0,
				800,
				800,
				0,
				0,
				800,
				800,
				gl::DEPTH_BUFFER_BIT,
				gl::NEAREST,
			);
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

			gl::DepthFunc(gl::LESS);
		}

		window.post_loop();
	}
}
