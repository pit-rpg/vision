extern crate gltf;
extern crate specs;
extern crate byteorder;
extern crate regex;
extern crate uuid;

use std::io::{Cursor, SeekFrom};
use self::byteorder::{LittleEndian, ReadBytesExt};
use self::uuid::Uuid;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::string::ToString;
use std::collections::HashSet;
use std::{
	fs,
	// io
};
use std::path::Path;
use std::path::PathBuf;
use std::boxed::Box;
use std::ops::Range;
use std::error::Error as StdError;
use math::{
	Vector,
	Vector2,
	Vector3,
	Vector4,
	Matrix2,
	Matrix3,
	Matrix4,
};


use self::gltf::{
	accessor::{
		Accessor,
		DataType,
		Dimensions,
	},
	mesh::{
		Semantic,
		Reader,
		util::ReadTexCoords,
		util::ReadColors,
	},
	buffer::{
		Source,
	},
	// material::{
	// 	Material,
	// },
	// image,
	image,
	Document,
};

use self::specs::{
	World,
	Builder,
	Entity
};

use core::{
	Transform,
	BufferData,
	BufferAttribute,
	BufferGeometry,
	BufferType,
	Texture2D,
	Material,
	SharedMaterial,
	SharedGeometry,
	EntityRelations,
	ShaderTag,
	ShaderProgram,
	TextureData,
	TextureColorType,
	Wrapping,
	MagFilter,
	MinFilter,
	SharedTexture2D,
	Uniform,
};

struct Context {
	path: PathBuf,
	doc: Document,
	images: Vec<TextureData>,
	textures: Vec<SharedTexture2D>,
	buffers: Vec<gltf::buffer::Data>,
	materials: Vec<SharedMaterial>,
	defaultMaterial: SharedMaterial,
}


pub fn load_gltf(world: &mut World, path: PathBuf) -> Result<Entity, Box<StdError>> {
	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");

	let (doc, buffers, images) = gltf::import(path.clone())?;

	let images: Vec<TextureData> = images
		.iter()
		.map(|e| TextureData::from(e) )
		.collect();

	let textures: Vec<SharedTexture2D> = doc.textures()
		.map(|e|{
			let index = e.source().index();
			let mut texture = Texture2D::from(e);
			texture.set_texture_data( Some(images[ index ].clone()) );
			SharedTexture2D::new(texture)
		})
		.collect();

	println!("<><><><><>==========++++++==========<><><><><>");
	let materials = doc.materials()
		.map(|in_mat| {
			let pbr = in_mat.pbr_metallic_roughness();
			let mut mat = Material::new_mesh_standard();

			let color_f = pbr.base_color_factor();
			let diffuse = Vector3::new_from_array(&color_f);
			let alpha = color_f[3];
			let roughness = pbr.roughness_factor();
			let metalness = pbr.metallic_factor();

			mat.set_uniform("diffuse", &Uniform::Vector3(diffuse));
			mat.set_uniform("roughness", &Uniform::Float(roughness));
			mat.set_uniform("metalness", &Uniform::Float(metalness));
			// mat.set_uniform("specular", &Uniform::Vector3(Vector3::new_one()));


			// println!("{:?}", pbr);

			SharedMaterial::new(mat)
		})
		.collect();
	println!("<><><><><>==========++++++==========<><><><><>");

	let context = Context {
		defaultMaterial: SharedMaterial::new(Material::new_normal()),
		materials,
		textures,
		doc,
		buffers,
		images,
		path: path.clone(),
	};

	let root = world.create_entity()
		.with(Transform::default())
		.build();

	for scene in context.doc.scenes() {
		print!("Scene {}", scene.index());
		// #[cfg(feature = "names")]
		print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
		println!();
		for node in scene.nodes() {
			load_node(world, &node, &context, 1, root);
			// print_tree(&node, 1);
		}
	}

	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");

	Ok(root)
}


fn load_node(world: &mut World, node: &gltf::Node, context: &Context, depth: i32, parent: Entity) {
	print!(" Node {}", node.index());
	print!(" ({})", node.name().unwrap_or("<dimensions {:?}>"));
	println!();

	// Transform
	let matrix = Matrix4::from_column_row_array( node.transform().matrix() );
	let transform = Transform::from_matrix(matrix);
	// / Transform

	// Mesh
	let meshes = node.mesh().map(|mesh| {
		println!(" -> Mesh: {} {}", mesh.index(), mesh.name().unwrap_or("<Unnamed>"));

		let primitives: Vec<_> = mesh.primitives()
			.map(|primitive| {
				println!();

				let reader = primitive.reader(|buffer| Some(&context.buffers[buffer.index()]));
				let mut shader_tags = HashSet::new();

				let indices = reader
					.read_indices()
					.map(|read_indices| {
						read_indices.into_u32().collect::<Vec<_>>()
					});


				let attributes: Vec<_> = primitive.attributes()
					.map(|(semantic, accessor)|{
						println!("ATTRIBUTES {:?} =================================", semantic);

						let data = match semantic {
							Semantic::Positions => {

								shader_tags.insert(ShaderTag::B_Position);


								let positions: Vec<_> = reader.read_positions()
									.expect("cant find positions")
									.map(|v| Vector3::new( v[0], v[1], v[2] ) )
									.collect();
								BufferData::Vector3(positions)
							}
							Semantic::Normals => {

								shader_tags.insert(ShaderTag::B_Normal);

								let normals: Vec<_> = reader.read_normals()
									.expect("cant find normals")
									.map(|v| Vector3::new( v[0], v[1], v[2] ) )
									.collect();
								BufferData::Vector3(normals)
							}
							Semantic::TexCoords(n) => {
								let en = reader.read_tex_coords(n).expect("cant find uv");

								shader_tags.insert(ShaderTag::B_UV);

								let uv: Vec<_> = match en {
									ReadTexCoords::U8(iter)=>{
										iter.map(|e| Vector2::new(e[0] as f32, e[1] as f32) ).collect()
									}
									ReadTexCoords::U16(iter)=>{
										iter.map(|e| Vector2::new(e[0] as f32, e[1] as f32) ).collect()
									}
									ReadTexCoords::F32(iter)=>{
										iter.map(|e| Vector2::new(e[0], e[1]) ).collect()
									}
								};
								BufferData::Vector2(uv)
							}
							Semantic::Colors(n) => {
								let en = reader.read_colors(n).expect("cant find colors");


								match en {
									ReadColors::RgbU8(iter) => {
										shader_tags.insert(ShaderTag::B_Color_3);
										let color: Vec<_> = iter.map(|e| Vector3::new(e[0] as f32, e[1] as f32, e[2] as f32) ).collect();
										BufferData::Vector3(color)
									},
									ReadColors::RgbU16(iter) => {
										shader_tags.insert(ShaderTag::B_Color_3);
										let color: Vec<_> = iter.map(|e| Vector3::new(e[0] as f32, e[1] as f32, e[2] as f32) ).collect();
										BufferData::Vector3(color)
									},
									ReadColors::RgbF32(iter) => {
										shader_tags.insert(ShaderTag::B_Color_3);
										let color: Vec<_> = iter.map(|e| Vector3::new( e[0], e[1], e[2]) ).collect();
										BufferData::Vector3(color)
									},
									ReadColors::RgbaU8(iter) => {
										shader_tags.insert(ShaderTag::B_Color_4);
										let color: Vec<_> = iter.map(|e| Vector4::new( e[0] as f32, e[1] as f32, e[2] as f32, e[3] as f32 ) ).collect();
										BufferData::Vector4(color)
									},
									ReadColors::RgbaU16(iter) => {
										shader_tags.insert(ShaderTag::B_Color_4);
										let color: Vec<_> = iter.map(|e| Vector4::new( e[0] as f32, e[1] as f32, e[2] as f32, e[3] as f32 ) ).collect();
										BufferData::Vector4(color)
									},
									ReadColors::RgbaF32(iter) => {
										shader_tags.insert(ShaderTag::B_Color_4);
										let color: Vec<_> = iter.map(|e| Vector4::new( e[0], e[1], e[2], e[3] ) ).collect();
										BufferData::Vector4(color)
									},
								}
							}
							Semantic::Joints(_) => {unimplemented!()}
							Semantic::Tangents => {
								let tangents: Vec<_> = reader.read_tangents()
									.expect("cant find tangents")
									.map(|v| Vector3::new( v[0], v[1], v[2] ) )
									.collect();
								BufferData::Vector3(tangents)
							}
							Semantic::Weights(_) => {unimplemented!()}
						};

						BufferAttribute {
							data,
							buffer_type: semantic.tp_buffer_type(),
							dynamic: false,
							normalized: accessor.normalized(),
						}
					})
					.collect();

				println!();

				let mut geom = BufferGeometry::new();
				attributes.into_iter().for_each(|e| {geom.add_buffer_attribute(e);} );
				indices.map(|data| {geom.set_indices(data)} );
				(geom, shader_tags, primitive.material().index())
			})
			.collect();
			primitives
	});
	// /Mesh

	let current = world.create_entity()
		.with(transform)
		.build();

	world.add_child(parent, current);
	let parent = current;

	let mut child_node = parent.clone();

	if let Some(meshes) = meshes {
		// println!("++++++++++++++++++++++++++++++++++++++++++");
		// println!("++++++++++++++++++++++++++++++++++++++++++");
		// meshes.iter().for_each(|mesh|{
		// 	mesh.attributes.iter().for_each(|attr|{
		// 		println!("NAME: {:?}", attr.buffer_type);
		// 	});
		// });
		// println!("++++++++++++++++++++++++++++++++++++++++++");
		// println!("++++++++++++++++++++++++++++++++++++++++++");


		for (mesh, mut shader_tags, material_index) in meshes {
			// let mut mat = Material::new_mesh_standard();
			let mut shard_mat = match material_index {
				None => context.defaultMaterial.clone(),
				Some(index) => context.materials[index].clone(),
			};
			// mat.set_uniform("diffuse", &Uniform::Vector3(Vector3::new_one()));
			// mat.set_uniform("specular", &Uniform::Vector3(Vector3::new_one()));
			// mat.set_uniform("roughness", &Uniform::Float(1.0));
			// mat.set_uniform("metalness", &Uniform::Float(0.0));
			// mat.set_uniform("ambientLightColor", &Uniform::Vector3(Vector3::new(0.0,0.0,0.0)));


			{
				let mut mat = shard_mat.lock().unwrap();
				let tags = mat.get_tags_mut();
				tags.extend(shader_tags.drain());

				println!("_______+++++++++++++___________++++++++_________++++++++_______");
				println!("{:?}", tags);
			}


			// let shard_mat = SharedMaterial::new(mat);
			let e  = world.create_entity()
				// .with(transform.clone())
				.with(Transform::default())
				.with(shard_mat)
				.with(SharedGeometry::new(mesh))
				.build();
			world.add_child(current, e);
		}
	}

	// println!();

	for child in node.children() {
		load_node(world, &child, context, depth + 1, current);
	}
}


trait SemanticToBufferType {
	fn tp_buffer_type(&self) -> BufferType;
}

impl SemanticToBufferType for Semantic {
	fn tp_buffer_type(&self) -> BufferType {
		match self {
			Semantic::Positions => BufferType::Position,
			Semantic::Normals => BufferType::Normal,
			Semantic::Tangents => BufferType::Tangent,
			Semantic::TexCoords(i) => BufferType::UV(*i as usize),
			Semantic::Colors(i) => BufferType::Color(*i as usize),
			Semantic::Joints(i) => BufferType::Joint(*i as usize),
			Semantic::Weights(i) => BufferType::Weight(*i as usize),
		}
	}
}

impl From<&image::Data> for TextureData {
	fn from(data: &image::Data) -> Self {
		let color_type = match data.format {
			image::Format::R8 => TextureColorType::R(8),
			image::Format::R8G8 => TextureColorType::RG(8),
			image::Format::R8G8B8 => TextureColorType::RGB(8),
			image::Format::R8G8B8A8 => TextureColorType::RGBA(8),
		};

		TextureData{
			width: data.width,
			height: data.height,
			color_type,
			data: data.pixels.clone(),
		}
	}
}

impl From<gltf::texture::WrappingMode> for Wrapping {
	fn from(data: gltf::texture::WrappingMode) -> Self {
		match data {
			gltf::texture::WrappingMode::ClampToEdge => Wrapping::ClampToEdge,
			gltf::texture::WrappingMode::MirroredRepeat => Wrapping::MirroredRepeat,
			gltf::texture::WrappingMode::Repeat => Wrapping::Repeat,
		}
	}
}

impl From<gltf::texture::MagFilter> for MagFilter {
	fn from(data: gltf::texture::MagFilter) -> Self {
		match data {
			gltf::texture::MagFilter::Linear => MagFilter::Linear,
			gltf::texture::MagFilter::Nearest => MagFilter::Nearest,
		}
	}
}

impl From<gltf::texture::MinFilter> for MinFilter {
	fn from(data: gltf::texture::MinFilter) -> Self {
		match data {
			gltf::texture::MinFilter::Nearest => MinFilter::Nearest,
			gltf::texture::MinFilter::NearestMipmapLinear => MinFilter::NearestMipmapLinear,
			gltf::texture::MinFilter::NearestMipmapNearest => MinFilter::NearestMipmapNearest,
			gltf::texture::MinFilter::Linear => MinFilter::Linear,
			gltf::texture::MinFilter::LinearMipmapLinear => MinFilter::LinearMipmapLinear,
			gltf::texture::MinFilter::LinearMipmapNearest => MinFilter::LinearMipmapNearest,
		}
	}
}

impl From<Option<gltf::texture::MinFilter>> for MinFilter {
	fn from(data: Option<gltf::texture::MinFilter>) -> Self {
		data.map_or(MinFilter::LinearMipmapLinear, |e| e.into())
	}
}

impl From<Option<gltf::texture::MagFilter>> for MagFilter {
	fn from(data: Option<gltf::texture::MagFilter>) -> Self {
		data.map_or(MagFilter::Linear, |e| e.into())
	}
}


impl From<gltf::Texture<'_>> for Texture2D {
	fn from(data: gltf::texture::Texture) -> Self {
		let sampler = data.sampler();

		let mut elem = Texture2D::default();
		elem.wrapping_x = Wrapping::from(sampler.wrap_s());
		elem.wrapping_y = Wrapping::from(sampler.wrap_t());
		elem.min_filter = sampler.min_filter().into();
		elem.mag_filter = sampler.mag_filter().into();
		elem.path = data.name().map(|e| e.to_string() );

		elem
	}
}