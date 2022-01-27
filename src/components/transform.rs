use nalgebra::{Matrix4, Rotation3, Scale3, Vector3};

pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Rotation3<f32>,
	pub scale: Scale3<f32>,

	pub forward: Vector3<f32>,
	pub right: Vector3<f32>,
	pub up: Vector3<f32>,
}

impl Default for Transform {
	fn default() -> Transform {
		Transform {
			position: Vector3::identity(),
			rotation: Rotation3::identity(),
			scale: Scale3::identity(),
			forward: -Vector3::z(),
			right: Vector3::x(),
			up: Vector3::y(),
		}
	}
}

impl Transform {
	pub fn get_matrix(&self) -> Matrix4<f32> {
		let mut mat = Matrix4::<f32>::new_translation(&self.position);
		mat = mat * &self.rotation.to_homogeneous();

		return mat;
	}
}
