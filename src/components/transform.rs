use glm;
use nalgebra::{vector, Matrix4, Rotation3, Vector3};

pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Rotation3<f32>,
	pub scale: Vector3<f32>,

	pub forward: Vector3<f32>,
	pub right: Vector3<f32>,
	pub up: Vector3<f32>,
}

impl Default for Transform {
	fn default() -> Transform {
		Transform {
			position: Vector3::identity(),
			rotation: Rotation3::identity(),
			scale: vector!(1.0, 1.0, 1.0),
			forward: Vector3::z(),
			right: Vector3::x(),
			up: Vector3::y(),
		}
	}
}

impl Transform {
	pub fn get_matrix(&self) -> Matrix4<f32> {
		let mut mat = Matrix4::<f32>::new_translation(&self.position);
		mat = mat * &self.rotation.to_homogeneous();
		mat = glm::scale(&mat, &self.scale);

		return mat;
	}

	pub fn update_directions(&mut self) {
		self.forward = self.rotation.transform_vector(&Vector3::z()).normalize();
		self.right = self.forward.cross(&Vector3::y()).normalize();
		self.up = self.right.cross(&self.forward).normalize();
	}

	pub fn translate(&mut self, tr: Vector3<f32>) {
		self.position = self.position + tr;
	}

	pub fn rotate(&mut self, rt: Vector3<f32>) {
		let rot_mat = Rotation3::from_euler_angles(rt.x, rt.y, rt.z);
		self.rotation = self.rotation * rot_mat;
	}

	pub fn rotate_euler(&mut self, roll: f32, pitch: f32, yaw: f32) {
		let rot_mat = Rotation3::from_euler_angles(roll, pitch, yaw);
		self.rotation = self.rotation * rot_mat;
	}
}
