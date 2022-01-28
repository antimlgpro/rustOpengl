use nalgebra::{Matrix4, Perspective3, Point3};

use crate::components::Transform;

pub struct Camera {
	pub mouse_sensitivity: f32,
	pub fov: f32,
	pub aspect_ratio: f32,

	pub projection: Matrix4<f32>,
	pub view: Matrix4<f32>,
}

impl Default for Camera {
	fn default() -> Camera {
		let mut cam = Camera {
			mouse_sensitivity: 0.1,
			fov: 90.0,
			aspect_ratio: 1.0,
			projection: Matrix4::default(),
			view: Matrix4::default(),
		};
		cam.update_projection();

		cam
	}
}

impl Camera {
	pub fn update_projection(&mut self) {
		let vertical = 2.0 * ((self.fov / 2.0).tan() * self.aspect_ratio).atan();
		let projection: Perspective3<f32> =
			Perspective3::new(self.aspect_ratio, vertical, 0.1, 100.0);

		self.projection = projection.to_homogeneous();
	}

	pub fn update_view(&mut self, tf: &Transform) {
		let pos: &Point3<f32> = &tf.position.into();
		let sm: Point3<f32> = (tf.position + tf.forward).into();

		self.view = Matrix4::look_at_rh(pos, &sm, &tf.up);
	}
}
