mod frame;
mod rendering;

pub mod error;

/// Contains window modules
#[path = "window.rs"]
mod wind;
pub mod window {
	pub use super::frame::*;
	pub use super::wind::*;
}

/// Contains modules used in rendering
pub mod render {
	/// Contains core modules for rendering
	pub mod core {
		pub use super::super::rendering::{
			mesh, shader, Loader, Material, Texture, TextureOptions, UniformManager,
		};
	}

	/// Contains buffer object modules
	pub mod buffers {
		pub use super::super::rendering::{FrameBuffer, RenderBuffer, UniformBuffer};
	}
}
