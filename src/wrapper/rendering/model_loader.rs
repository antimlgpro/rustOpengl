use crate::{
	util::calculate_normals,
	wrapper::{GLError, Mesh, Vertex},
};
use nalgebra::{vector, Vector3};
use std::path::Path;
use tobj;

pub struct Loader {}

impl Loader {
	pub fn load(path: &str) -> Result<Vec<Mesh>, GLError> {
		let path = Path::new(path);

		let options = &tobj::LoadOptions {
			single_index: true,
			triangulate: true,
			..Default::default()
		};
		let obj = tobj::load_obj(path, options);

		let mut meshes: Vec<Mesh> = Vec::new();

		let (models, _) = obj.unwrap();
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

			/* for i in (0..num_vertices).step_by(3) {
				let mut position = Vector3::<f32>::default();
				let mut normal = Vector3::<f32>::default();

				if p.len() == num_vertices {
					position = vector!(p[i], p[i + 1], p[i + 2])
				}

				if n.len() == num_vertices {
					normal = vector!(n[i], n[i + 1], n[i + 2]);
				}

				vertices.push(Vertex {
					position,
					normal,
					..Vertex::default()
				});
			} */

			let mesh = match Mesh::new(vertices, indices) {
				Ok(e) => e,
				Err(err) => {
					return Err(err);
				}
			};
			meshes.push(mesh);
		}

		return Ok(meshes);
	}
}
