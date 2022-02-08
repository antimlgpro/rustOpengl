use crate::{
	util::calculate_normals,
	wrapper::{
		error::GLError,
		render::core::{
			mesh::{Mesh, Vertex},
			Texture, TextureOptions,
		},
	},
};
use nalgebra::{vector, Vector3};
use std::path::Path;
use tobj;

pub struct Loader {
	textures_loaded: Vec<Texture>,
}

impl Loader {
	pub fn load(path: &str) -> Result<Vec<Mesh>, GLError> {
		let mut loader = Loader {
			textures_loaded: Vec::new(),
		};

		let path = Path::new(path);

		let options = &tobj::LoadOptions {
			single_index: true,
			triangulate: true,
			..Default::default()
		};
		let obj = tobj::load_obj(path, options);

		let mut meshes: Vec<Mesh> = Vec::new();
		let (models, materials) = obj.expect("Failed to load OBJ file");

		for model in models {
			let mesh = model.mesh;
			let num_vertices = mesh.positions.len();

			let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
			let indices: Vec<u32> = mesh.indices.clone();

			let (p, n) = (&mesh.positions, &mesh.normals);

			let mut positions: Vec<Vector3<f32>> = Vec::new();
			for i in (0..num_vertices).step_by(3) {
				positions.push(vector!(p[i], p[i + 1], p[i + 2]));
			}

			let mut normals: Vec<Vector3<f32>> = Vec::new();
			if n.len() != num_vertices {
				normals = calculate_normals(&positions, &indices);
			} else {
				for i in (0..num_vertices).step_by(3) {
					normals.push(vector!(n[i], n[i + 1], n[i + 2]));
				}
			}

			for i in 0..positions.len() {
				vertices.push(Vertex {
					position: positions[i],
					normal: normals[i],
					..Vertex::default()
				});
			}

			let mut textures = Vec::new();
			match materials {
				Err(_) => {
					// Default blank texture
					textures.push(Texture::blank_texture(
						"texture_diffuse",
						&TextureOptions {
							width: 128,
							height: 128,
							internal_format: gl::RGB,
							format: gl::RGB,
							type_: gl::UNSIGNED_BYTE,
						},
					));
				}
				Ok(ref mat) => {
					if let Some(material_id) = mesh.material_id {
						let material = &mat[material_id];
						// 1. diffuse map
						if !material.diffuse_texture.is_empty() {
							let texture = loader.load_material_texture(
								&material.diffuse_texture,
								"texture_diffuse",
							);
							textures.push(texture);
						}
						// 2. specular map
						if !material.specular_texture.is_empty() {
							let texture = loader.load_material_texture(
								&material.specular_texture,
								"texture_specular",
							);
							textures.push(texture);
						}
						// 3. normal map
						if !material.normal_texture.is_empty() {
							let texture = loader
								.load_material_texture(&material.normal_texture, "texture_normal");
							textures.push(texture);
						}
						// NOTE: no height maps
					}
				}
			}

			let mesh = match Mesh::new(vertices, indices, textures) {
				Ok(e) => e,
				Err(err) => {
					return Err(err);
				}
			};
			meshes.push(mesh);
		}

		return Ok(meshes);
	}

	fn load_material_texture(&mut self, path: &str, type_name: &str) -> Texture {
		// check for duplicates
		let texture = self.textures_loaded.iter().find(|t| t.path == path);
		if let Some(texture) = texture {
			return texture.clone();
		}

		let texture = Texture::from_file(type_name, path);

		self.textures_loaded.push(texture.clone());
		texture
	}
}
