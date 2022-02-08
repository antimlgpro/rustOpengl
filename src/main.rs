#![allow(dead_code)]
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use legion::*;
use nalgebra::{vector, Matrix4, Rotation3};
use rand::Rng;
use std::{mem::size_of, os::raw::c_void};

mod components;
mod util;
mod wrapper;
use components::*;
use util::radians;
use wrapper::{
	error_callback, FrameBuffer, Loader, Material, RenderBuffer, Shader, TextureBuffer,
	UniformBuffer, UniformManager, Window, WindowSettings,
};

#[system(for_each)]
fn render_model(tf: &Transform, rend: &Renderable, #[resource] unif_man: &mut UniformManager) {
	let mesh = &rend.mesh;
	let shader = &rend.material.shader;
	let model = tf.get_matrix();

	shader.use_program();
	shader.set_mat4("model", &model);
	shader.set_mat4(
		"normal_mat",
		&model.try_inverse().expect("Could not inverse?").transpose(),
	);
	rend.material.set_all(unif_man);

	mesh.draw();
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
	let mut window = Window::new(WindowSettings::default()).default_setup();
	window.debug_message_callback(Some(error_callback));

	let mut world = legion::World::default();
	let mut resources = Resources::default();
	let mut schedule = Schedule::builder()
		.add_thread_local(update_camera_system())
		//.add_thread_local(render_model_system())
		.build();

	let std_pass = Shader::new("shaders/deferred/std.vs", "shaders/deferred/std.fs").unwrap();
	let light_pass = Shader::new("shaders/deferred/light.vs", "shaders/deferred/light.fs").unwrap();

	//let mut uniform_man = UniformManager::new();
	let point = 0;
	UniformBuffer::set_uniform_block(&std_pass, "Matrices", point);
	let ubo_matrices = match UniformBuffer::create_buffer(point, 2 * size_of::<Matrix4<f32>>()) {
		Ok(e) => e,
		Err(e) => {
			panic!("Create_buffer: {}", e);
		}
	};

	let mut loaded = match Loader::load("models/teapot.obj") {
		Ok(m) => m,
		Err(e) => {
			panic!("Loader: {}", e);
		}
	};
	let mesh = loaded.remove(0);

	//resources.insert(uniform_man);
	resources.insert(ubo_matrices);

	let test_mat = Material::new(std_pass, vec![("albedo", vector!(1.0, 1.0, 1.0))], vec![]);

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

	let teapot = world.push((
		Transform {
			position: vector![0.0, 0.0, 0.0],

			..Transform::default()
		},
		Renderable {
			material: test_mat,
			mesh,
		},
	));

	let fbo_closure = move || {
		let (screen_width, screen_height) = (window.settings.width, window.settings.height);

		let tex_buf1 = TextureBuffer {
			name: "position".to_owned(),
			width: screen_width,
			height: screen_height,
			internal_format: gl::RGBA16F,
			format: gl::RGBA,
			type_: gl::FLOAT,
			attachment: gl::COLOR_ATTACHMENT0,
		};
		let tex_buf2 = TextureBuffer {
			name: "normal".to_owned(),
			width: screen_width,
			height: screen_height,
			internal_format: gl::RGBA16F,
			format: gl::RGBA,
			type_: gl::FLOAT,
			attachment: gl::COLOR_ATTACHMENT1,
		};
		let tex_buf3 = TextureBuffer {
			name: "albedo_spec".to_owned(),
			width: screen_width,
			height: screen_height,
			internal_format: gl::RGBA16F,
			format: gl::RGBA,
			type_: gl::UNSIGNED_BYTE,
			attachment: gl::COLOR_ATTACHMENT2,
		};

		let mut g_buffer = FrameBuffer::new();

		g_buffer.gen_texture_buffer(tex_buf1);
		g_buffer.gen_texture_buffer(tex_buf2);
		g_buffer.gen_texture_buffer(tex_buf3);

		g_buffer.draw_buffers();
		let rbo = RenderBuffer::new(screen_width, screen_height);
		rbo.framebuffer_renderbuffer();
		g_buffer.finish().unwrap();

		g_buffer
	};
	let g_buffer = fbo_closure();

	let quad_vertices: [f32; 24] = [
		-1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0,
		-1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
	];

	let mut quad_vao: u32 = 0;
	let mut quad_vbo: u32 = 0;
	unsafe {
		gl::GenVertexArrays(1, &mut quad_vao);
		gl::GenBuffers(1, &mut quad_vbo);
		gl::BindVertexArray(quad_vao);
		gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
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
	}

	let mut cube_texture: u32 = 0;
	unsafe {
		let width = 128;
		let height = 128;

		let capacity = width * height;
		let mut data: Vec<u32> = vec![0; capacity * 3];
		for y in 0..width {
			for x in 0..height {
				let offset = y * width + x;
				data[offset * 3 + 0] = 255;
				data[offset * 3 + 1] = 255;
				data[offset * 3 + 2] = 255;
			}
		}

		gl::GenTextures(1, &mut cube_texture);
		gl::BindTexture(gl::TEXTURE_2D, cube_texture);

		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGB as i32,
			width as i32,
			height as i32,
			0,
			gl::RGB,
			gl::UNSIGNED_BYTE,
			data.as_ptr() as *const c_void,
		);
		gl::GenerateMipmap(gl::TEXTURE_2D);
	}

	std_pass.use_program();
	light_pass.set_int("texture1", 0);

	light_pass.use_program();
	light_pass.set_int("gPosition", 0);
	light_pass.set_int("gNormal", 1);
	light_pass.set_int("gAlbedoSpec", 2);

	while !window.should_close() {
		window.pre_loop();

		//let frame = window.get_frame();
		//frame.clear_color(0.1, 0.1, 0.1, 1.0);
		//frame.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

		let time = window.get_time();
		let delta_time = time.delta_time;

		schedule.execute(&mut world, &mut resources);

		unsafe {
			g_buffer.bind();
			//gl::Enable(gl::DEPTH_TEST);
			gl::ClearColor(1.0, 1.0, 1.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}

		let mut query = <(&mut Transform, &Renderable)>::query();
		for (tf, rend) in query.iter_mut(&mut world) {
			let mesh = &rend.mesh;
			let shader = &rend.material.shader;
			let model = tf.get_matrix();

			shader.use_program();
			shader.set_mat4("model", &model);
			shader.set_mat4(
				"normal_mat",
				&model.try_inverse().expect("Could not inverse?").transpose(),
			);
			//rend.material.set_all(&mut uniform_man);

			tf.rotate_euler(0.0, radians(90.0) * delta_time, 0.0);

			unsafe {
				gl::ActiveTexture(gl::TEXTURE0);
				gl::BindTexture(gl::TEXTURE_2D, cube_texture);
			}

			mesh.draw();
		}

		unsafe {
			g_buffer.unbind();
			//gl::Disable(gl::DEPTH_TEST);
			gl::ClearColor(1.0, 1.0, 1.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); //

			let buf = g_buffer.get_buffer("position");
			let buf2 = g_buffer.get_buffer("normal");
			let buf3 = g_buffer.get_buffer("albedo_spec");

			gl::ActiveTexture(gl::TEXTURE0);
			gl::BindTexture(gl::TEXTURE_2D, *buf);
			gl::ActiveTexture(gl::TEXTURE1);
			gl::BindTexture(gl::TEXTURE_2D, *buf2);
			gl::ActiveTexture(gl::TEXTURE2);
			gl::BindTexture(gl::TEXTURE_2D, *buf3);

			light_pass.use_program();
			light_pass.set_vector3("lightPos", &vector![0.0, 3.0, 0.0]);
			light_pass.set_vector3("viewPos", &vector![0.0, 3.0, -4.5]);

			gl::BindVertexArray(quad_vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 6);
		}

		window.post_loop();
	}
}
