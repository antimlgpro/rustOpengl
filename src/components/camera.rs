use nalgebra::{Matrix4, Perspective3, Point3};

use crate::components::Transform;

pub struct Camera {
	pub mouse_sensitivity: f32,
	pub fov: f32,
	pub aspect_ratio: f32,

	pub projection_view: Matrix4<f32>,
}

impl Default for Camera {
	fn default() -> Camera {
		Camera {
			mouse_sensitivity: 0.1,
			fov: 90.0,
			aspect_ratio: 1.0,
			projection_view: Matrix4::default(),
		}
	}
}

impl Camera {
	pub fn get_view_matrix(&self, tf: &Transform) -> Matrix4<f32> {
		let pos: &Point3<f32> = &tf.position.into();
		let sm: Point3<f32> = (tf.position + tf.forward).into();
		//let sm = Point3::new(0.0, 0.0, 0.0);
		return Matrix4::look_at_rh(pos, &sm, &tf.up);
	}

	pub fn update_projection_view(&mut self, transform: &Transform) {
		let vertical = 2.0 * ((self.fov / 2.0).tan() * self.aspect_ratio).atan();
		let projection: Perspective3<f32> =
			Perspective3::new(self.aspect_ratio, vertical, 0.1, 100.0);

		let view = self.get_view_matrix(transform);
		self.projection_view = projection.as_matrix() * view;
	}
}
