use crate::wrapper::{Shader, UniformManager};
use nalgebra::{Matrix4, Vector3};
use std::collections::HashMap;

pub trait ValidTypes {
	fn to_vector(self) -> Vector3<f32>;
}
impl ValidTypes for Vector3<f32> {
	fn to_vector(self) -> Vector3<f32> {
		self
	}
}

pub struct Material {
	pub shader: Shader,
	pub uniforms: HashMap<String, Vector3<f32>>,
	pub attributes: Vec<String>,
}

impl Material {
	pub fn new<T: ValidTypes>(
		shader: Shader,
		uniforms: Vec<(&str, T)>,
		attributes: Vec<&str>,
	) -> Self {
		let mut mat = Material {
			shader,
			uniforms: HashMap::new(),
			attributes: Vec::new(),
		};

		for name in attributes {
			mat.attributes.push(name.to_owned());
		}

		for (name, value) in uniforms {
			mat.uniforms.insert(name.to_owned(), value.to_vector());
		}

		mat
	}

	pub fn add_uniform<T: ValidTypes>(&mut self, name: &str, uniform: T) {
		self.uniforms.insert(name.to_owned(), uniform.to_vector());
	}

	pub fn remove_uniform(&mut self, name: &str) {
		self.uniforms.remove(name);
	}

	pub fn change_uniform<T: ValidTypes>(&mut self, name: &str, uniform: T) {
		self.uniforms.insert(name.to_owned(), uniform.to_vector());
	}

	pub fn add_attribute(&mut self, name: &str) {
		self.attributes.push(name.to_owned());
	}

	pub fn remove_attribute(&mut self, name: &str) {
		if let Some(pos) = self.attributes.iter().position(|x| *x == name) {
			self.attributes.remove(pos);
		}
	}

	pub fn set_all(&self, unif_man: &mut UniformManager) {
		self.set_uniforms_vec3(unif_man);
		self.set_uniforms_mat4(unif_man);

		for (name, val) in &self.uniforms {
			self.shader.set_vector3(name, val);
		}
	}

	fn set_uniforms_vec3(&self, unif_man: &mut UniformManager) {
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

	fn set_uniforms_mat4(&self, unif_man: &mut UniformManager) {
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
