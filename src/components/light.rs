use nalgebra::Vector3;

pub struct Light {
	pub position: Vector3<f32>,
	pub color: Vector3<f32>,

	pub quadratic: f32,
	pub linear: f32,
}

impl Light {
	pub fn new(position: Vector3<f32>, color: Vector3<f32>) -> Self {
		Light {
			position,
			color,
			quadratic: 1.8,
			linear: 0.7,
		}
	}
}
