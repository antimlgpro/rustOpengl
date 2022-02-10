use nalgebra::Vector3;

pub struct Light {
	pub color: Vector3<f32>,

	pub quadratic: f32,
	pub linear: f32,
}

impl Default for Light {
	fn default() -> Light {
		Light {
			color: Vector3::default(),
			quadratic: 1.8,
			linear: 0.7,
		}
	}
}
