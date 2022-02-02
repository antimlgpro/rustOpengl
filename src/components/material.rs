use crate::wrapper::{Shader, UniformManager};
use nalgebra::{Matrix4, Vector3};

pub struct Material {
	pub shader: Shader,
	pub attributes: Vec<String>,
}

impl Material {
	pub fn new(shader: Shader, attributes: Vec<&str>) -> Self {
		let mut mat = Material {
			shader,
			attributes: Vec::new(),
		};
		for name in attributes {
			mat.attributes.push(name.to_owned());
		}

		mat
	}

	pub fn add_attribute(&mut self, name: &str) {
		self.attributes.push(name.to_owned());
	}

	pub fn remove_attribute(&mut self, name: &str) {
		if let Some(pos) = self.attributes.iter().position(|x| *x == name) {
			self.attributes.remove(pos);
		}
	}

	pub fn set_uniforms_vec3(&self, unif_man: &mut UniformManager) {
		for name in &self.attributes {
			let val = match unif_man.get_uniform::<Vector3<f32>>(name) {
				Some(e) => e,
				None => {
					continue;
				}
			};
			self.shader.set_vector3(name, val);
		}
	}

	pub fn set_uniforms_mat4(&self, unif_man: &mut UniformManager) {
		for name in &self.attributes {
			let val = match unif_man.get_uniform::<Matrix4<f32>>(name) {
				Some(e) => e,
				None => {
					continue;
				}
			};
			self.shader.set_mat4(name, val);
		}
	}
}
