use crate::wrapper::render::core::shader::Shader;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct Material {
	pub shader: Shader,

	pub albedo: Vector3<f32>,
	pub metallic: f32,
	pub roughness: f32,
	pub ao: f32,
}

impl Material {
	pub fn new(
		shader: Shader,
		albedo: Vector3<f32>,
		metallic: f32,
		roughness: f32,
		ao: f32,
	) -> Material {
		Material {
			shader,
			albedo,
			metallic,
			roughness,
			ao,
		}
	}

	pub fn use_material(&self) {
		self.shader.set_vector3("material_albedo", &self.albedo);
		self.shader.set_float("material_metallic", self.metallic);
		self.shader.set_float("material_roughness", self.roughness);
		self.shader.set_float("material_ao", self.ao);
	}
}
