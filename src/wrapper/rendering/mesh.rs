use memoffset::offset_of;
use nalgebra::{Vector2, Vector3};
use std::{fmt, fmt::Display, mem::size_of, os::raw::c_void, ptr};

use crate::util::to_cstring;
use crate::wrapper::{
	error::GLError,
	render::core::{shader::Shader, Texture},
};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Vertex {
	pub position: Vector3<f32>,
	pub normal: Vector3<f32>,
	pub tex_coords: Vector2<f32>,
}

impl Default for Vertex {
	fn default() -> Vertex {
		Vertex {
			position: Vector3::default(),
			normal: Vector3::default(),
			tex_coords: Vector2::default(),
		}
	}
}

impl Display for Vertex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Positions: {} \
			Normal: {}",
			self.position, self.normal
		)
	}
}

#[derive(Clone)]
pub struct Mesh {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
	pub textures: Vec<Texture>,

	pub vao: u32,
	vbo: u32,
	ebo: u32,
}

// TODO: Add error checking for mesh construction
impl Mesh {
	pub fn new(
		vertices: Vec<Vertex>,
		indices: Vec<u32>,
		textures: Vec<Texture>,
	) -> Result<Mesh, GLError> {
		let mut mesh = Mesh {
			vertices,
			indices,
			textures,
			vao: 0,
			vbo: 0,
			ebo: 0,
		};

		mesh.setup();

		Ok(mesh)
	}

	pub fn draw(&self, shader: &Shader) {
		unsafe {
			// bind appropriate textures
			let mut diffuseNr = 0;
			let mut specularNr = 0;
			let mut normalNr = 0;
			let mut heightNr = 0;
			for (i, texture) in self.textures.iter().enumerate() {
				gl::ActiveTexture(gl::TEXTURE0 + i as u32);
				let name = &texture.type_name;
				let number = match name.as_str() {
					"texture_diffuse" => {
						diffuseNr += 1;
						diffuseNr
					}
					"texture_specular" => {
						specularNr += 1;
						specularNr
					}
					"texture_normal" => {
						normalNr += 1;
						normalNr
					}
					"texture_height" => {
						heightNr += 1;
						heightNr
					}
					_ => panic!("unknown texture type"),
				};
				// now set the sampler to the correct texture unit
				let sampler = to_cstring(format!("{}{}", name, number)).unwrap();
				gl::Uniform1i(
					gl::GetUniformLocation(shader.id, sampler.as_ptr()),
					i as i32,
				);
				// and finally bind the texture
				gl::BindTexture(gl::TEXTURE_2D, texture.id);
			}

			gl::BindVertexArray(self.vao);
			gl::DrawElements(
				gl::TRIANGLES,
				self.indices.len() as i32,
				gl::UNSIGNED_INT,
				ptr::null(),
			);
			gl::BindVertexArray(0);

			// Unbind textures
			gl::ActiveTexture(gl::TEXTURE0);
		}
	}

	fn setup(&mut self) {
		unsafe {
			gl::GenVertexArrays(1, &mut self.vao);
			gl::GenBuffers(1, &mut self.vbo);
			gl::GenBuffers(1, &mut self.ebo);

			gl::BindVertexArray(self.vao);
			// load data into vertex buffers
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

			let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
			let data = &self.vertices[0] as *const Vertex as *const c_void;
			gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
			let size = (self.indices.len() * size_of::<u32>()) as isize;
			let data = &self.indices[0] as *const u32 as *const c_void;
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

			// set the vertex attribute pointers
			let size = size_of::<Vertex>() as i32;
			// vertex Positions
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				0,
				3,
				gl::FLOAT,
				gl::FALSE,
				size,
				offset_of!(Vertex, position) as *const c_void,
			);
			// vertex normals
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				size,
				offset_of!(Vertex, normal) as *const c_void,
			);
			// vertex texcoords
			gl::EnableVertexAttribArray(2);
			gl::VertexAttribPointer(
				2,
				3,
				gl::FLOAT,
				gl::FALSE,
				size,
				offset_of!(Vertex, tex_coords) as *const c_void,
			);

			gl::BindVertexArray(0);
		}
	}
}
