use nalgebra::{Matrix4, Vector3};
use std::any::Any;
use std::collections::HashMap;

pub trait Valid {
	fn as_any(&self) -> &dyn Any;
}
impl Valid for Matrix4<f32> {
	fn as_any(&self) -> &dyn Any {
		self
	}
}
impl Valid for Vector3<f32> {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

/*
pub enum Valid {
	Vector3(Vector3<f32>),
	Matrix4(Matrix4<f32>),
}
*/

pub struct UniformManager {
	uniforms: HashMap<String, Box<dyn Valid>>,
}

impl UniformManager {
	pub fn new() -> Self {
		UniformManager {
			uniforms: HashMap::new(),
		}
	}

	pub fn add_uniform<T: Valid + 'static>(&mut self, name: &str, data: T) {
		self.uniforms.insert(name.to_owned(), Box::new(data));
	}

	pub fn get_uniform<T: Valid + 'static>(&mut self, name: &str) -> Option<&T> {
		let res = match self.uniforms.get_mut(name) {
			Some(e) => e,
			None => return None,
		};
		let a = match res.as_any().downcast_ref::<T>() {
			Some(e) => e,
			None => {
				return None;
			}
		};

		return Some(a);
	}

	pub fn set_uniform<T: Valid + 'static>(&mut self, name: &str, data: T) {
		self.uniforms.insert(name.to_owned(), Box::new(data));
	}
}
