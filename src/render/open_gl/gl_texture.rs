extern crate gl;
extern crate uuid;

use self::gl::types::*;
use self::uuid::Uuid;
use core::{Texture, TextureColorType};
use std::collections::HashMap;
use std::os::raw::c_void;

pub type GLTextureIDs = HashMap<Uuid, TextureId>;

#[derive(Debug)]
pub struct TextureId {
	pub id: GLuint,
	pub gl_texture_dimensions: u32,
}

impl Drop for TextureId {
	fn drop(&mut self) {
		println!("delete texture");

		gl_call!({
			// TODO remove textures
			gl::DeleteTextures(1, self.id as *const u32);
		});
	}
}

pub trait GLTexture {
	fn bind(&self, hash_map: &GLTextureIDs);
	fn unbind(&self);
}

fn to_gl_color_type(color_type: &TextureColorType) -> u32 {
	// TODO color depth support
	match color_type {
		TextureColorType::RGB(_) => gl::RGB,
		TextureColorType::RGBA(_) => gl::RGBA,
		TextureColorType::Gray(_) => gl::DEPTH_COMPONENT,
		_ => gl::RGBA,
	}
}

pub fn load_texture(texture: &Texture) -> Result<TextureId, ()> {
	println!("_/ LOAD TEXTURE______________________________",);

	let mut id: u32 = 0;
	let texture_data = texture.load().expect(&format!("Error cant load texture: {}", texture.path));
	let gl_texture_dimensions = if texture_data.height == 1 {
		gl::TEXTURE_1D
	} else {
		gl::TEXTURE_2D
	};

	println!("{:?}", texture_data.color_type);
	let color_type = to_gl_color_type(&texture_data.color_type);

	gl_call!({
		gl::GenTextures(1, &mut id);
		gl::BindTexture(gl_texture_dimensions, id);

		gl::TexImage2D(
			gl_texture_dimensions,
			0,
			color_type as i32,
			texture_data.width as i32,
			texture_data.height as i32,
			0,
			color_type,
			gl::UNSIGNED_BYTE,
			&texture_data.data[0] as *const u8 as *const c_void,
		);
		gl::GenerateMipmap(gl_texture_dimensions);
	});

	println!("__ LOAD TEXTURE______________________________",);

	Ok(TextureId {
		id,
		gl_texture_dimensions,
	})
}