use crate::util::to_cstring;
use crate::wrapper::render::core::shader::Shader;
use gl::types::*;
use image;
use image::DynamicImage::*;
use std::os::raw::c_void;
use std::path::Path;

pub struct TextureOptions {
	pub width: u32,
	pub height: u32,
	pub type_: GLenum,
	pub internal_format: GLenum,
	pub format: GLenum,
}

#[derive(Clone)]
pub struct Texture {
	pub id: u32,
	pub type_name: String,
	pub path: String,

	pub index: u32,
}

impl Texture {
	fn create_buffer(options: &TextureOptions, data: *const c_void) -> u32 {
		unsafe {
			let mut buf: u32 = 0;
			gl::GenTextures(1, &mut buf);

			gl::GenTextures(1, &mut buf);
			gl::BindTexture(gl::TEXTURE_2D, buf);
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				options.internal_format as i32,
				options.width as i32,
				options.height as i32,
				0,
				options.format,
				gl::UNSIGNED_BYTE,
				data,
			);
			return buf;
		}
	}

	pub fn from_file(type_name: &str, path: &str) -> Texture {
		let img = image::open(&Path::new(path)).expect("Texture failed to load");
		let img = img.flipv();
		let format = match img {
			ImageLuma8(_) => gl::RED,
			ImageLumaA8(_) => gl::RG,
			ImageRgb8(_) => gl::RGB,
			ImageRgba8(_) => gl::RGBA,
			_ => 0,
		};

		let data = img.as_bytes();

		let texture_id = unsafe {
			let options = TextureOptions {
				width: img.width(),
				height: img.height(),
				internal_format: format,
				format,
				type_: gl::UNSIGNED_BYTE,
			};
			let buf = Texture::create_buffer(&options, &data[0] as *const u8 as *const c_void);
			gl::GenerateMipmap(gl::TEXTURE_2D);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
			gl::TexParameteri(
				gl::TEXTURE_2D,
				gl::TEXTURE_MIN_FILTER,
				gl::LINEAR_MIPMAP_LINEAR as i32,
			);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

			buf
		};

		let texture = Texture {
			id: texture_id,
			type_name: type_name.to_owned(),
			path: path.to_owned(),
			index: 0,
		};

		texture
	}

	pub fn for_framebuffer(type_name: &str, index: u32, options: &TextureOptions) -> Texture {
		let texture_id = unsafe {
			let buf = Texture::create_buffer(options, std::ptr::null());
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			gl::FramebufferTexture2D(
				gl::FRAMEBUFFER,
				gl::COLOR_ATTACHMENT0 + index,
				gl::TEXTURE_2D,
				buf,
				0,
			);

			buf
		};

		Texture {
			id: texture_id,
			type_name: type_name.to_owned(),
			path: "".to_owned(),
			index,
		}
	}

	pub fn blank_texture(type_name: &str, options: &TextureOptions) -> Texture {
		let width = options.width as usize;
		let height = options.height as usize;

		let capacity = width * height;
		let mut data: Vec<u32> = vec![0; capacity * 3];
		for y in 0..width {
			for x in 0..height {
				let offset = y * width + x;
				data[offset * 3 + 0] = 255;
				data[offset * 3 + 1] = 255;
				data[offset * 3 + 2] = 255;
			}
		}

		let id = Texture::create_buffer(options, data.as_ptr() as *const c_void);
		unsafe {
			gl::GenerateMipmap(gl::TEXTURE_2D);
		}

		Texture {
			id,
			type_name: type_name.to_owned(),
			path: "".to_owned(),
			index: 0,
		}
	}
}

impl Texture {
	pub fn bind(&self, shader: &Shader, index: u32) {
		unsafe {
			let sampler = to_cstring(self.type_name.clone()).unwrap();
			gl::Uniform1i(
				gl::GetUniformLocation(shader.id, sampler.as_ptr()),
				index as i32,
			);
			gl::ActiveTexture(gl::TEXTURE0 + index);
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}

	// activates and binds texture
	pub fn activate(&self) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + self.index);
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}
}
