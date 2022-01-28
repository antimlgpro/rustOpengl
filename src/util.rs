use std::error::Error;
use std::ffi::CString;
use std::fs;

use nalgebra::{vector, Vector3};

pub fn load_to_string(path: &str) -> Result<String, Box<dyn Error>> {
	let content = fs::read_to_string(path)?;

	Ok(content)
}

pub fn to_cstring(s: String) -> Result<CString, Box<dyn Error>> {
	let cst = CString::new(s.as_bytes())?;
	Ok(cst)
}

pub fn radians(num: f32) -> f32 {
	return num.to_radians();
}

fn compute_face_normal(p1: Vector3<f32>, p2: Vector3<f32>, p3: Vector3<f32>) -> Vector3<f32> {
	let a = p3 - p2;
	let b = p1 - p2;

	return a.cross(&b).normalize();
}

pub fn calculate_normals(vertices: &Vec<Vector3<f32>>, indices: &Vec<u32>) -> Vec<Vector3<f32>> {
	let mut normals: Vec<Vector3<f32>> = vec![vector!(0.0, 0.0, 0.0); vertices.len()];

	for i in (0..indices.len()).step_by(3) {
		let a = vertices[indices[i] as usize];
		let b = vertices[indices[i + 1] as usize];
		let c = vertices[indices[i + 2] as usize];
		let normal = compute_face_normal(a, b, c);

		normals[indices[i] as usize] += normal;
		normals[indices[i + 1] as usize] += normal;
		normals[indices[i + 2] as usize] += normal;
	}

	for i in 0..normals.len() {
		normals[i] = normals[i].normalize();
	}
	return normals;
}

pub fn ping_pong(t: f32, length: f32) -> f32 {
	let t = repeat(t, length * 2.0);
	return length - (t - length).abs();
}

pub fn repeat(t: f32, length: f32) -> f32 {
	return t - (t / length).floor() * length;
}
