use crate::wrapper::render::core::{mesh::Mesh, Material};

pub struct Renderable {
	pub material: Material,
	pub mesh: Mesh,
}

impl Renderable {}
