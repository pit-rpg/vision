#[macro_use] extern crate project;

use std::f64::consts::PI as PI_f64;
use std::path::PathBuf;

use project::{
	core::{
		FrameOutput, Material, PerspectiveCamera, NodeData, Node, Light, LightType,
		SharedFrameBuffer, SharedGeometry, Transform, UniformName,
	},
	glutin::event_loop::ControlFlow,
	glutin::event::MouseScrollDelta,
	glutin::event::WindowEvent,
	glutin::event::Event,
	helpers::{geometry_generators, load_gltf},
	math::{Vector, Vector3, Vector4},
	render,
};


#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WindowState {
	pub pointer_pos: (f64, f64),
	pub pointer_pressed: (bool, bool, bool),
	pub pointer_wheel: f32,
	pub window_size: (u32, u32),
}

fn main() {
	let root = Node::new("root");

	let up = Vector3::new(0.0, 1.0, 0.0);
	let center = Vector3::new_zero();
	let mut radius = 20.0;
	let zoom_speed = 0.5;

	let geom_light = SharedGeometry::new(geometry_generators::sphere(0.5, 12, 12));

	let camera = {
		let mut camera = PerspectiveCamera::new();
		let mut transform_camera = Transform::default();
		transform_camera.position.z = 6.0;
		camera.view.enabled = false;

		root.add_child(
			NodeData::new("camera")
				.set_camera(camera)
				.set_transform(transform_camera)
				.to_shared()
		)
	};


	let mut frame_buffer = SharedFrameBuffer::new_color_map_output(512, 512);
	let mut buffer_plane = root.clone();
	{
		let buffer_texture = {
			let buffer = frame_buffer.lock().unwrap();
			buffer
				.frame_outputs
				.iter()
				.find(|item| {
					if let FrameOutput::SharedTexture2D(_) = item {
						true
					} else {
						false
					}
				})
				.unwrap()
				.clone()
		};

		if let FrameOutput::SharedTexture2D(texture) = buffer_texture {
			let mut mat = Material::new_mesh_standard();
			mat.set_uniform(UniformName::MapColor, texture);
			let material = mat.to_shared();

			let mut transform = Transform::default();
			transform.scale.set(1.0, 1.0, 1.0);
			// transform.rotation.x = 3.14 / 2.0;
			transform.position.y = 1.0;
			transform.position.x = -1.0;
			transform.position.z = -3.3;

			buffer_plane = NodeData::new("plane")
				.set_transform(transform)
				.set_geometry(geometry_generators::plane_buffer_geometry(
					1.0, 1.0, 1, 1,
				).to_shared())
				.set_material(material)
				.to_shared();

			// root.add_child(buffer_plane.clone());
		}
	}

	{
		let mut transform = Transform::default();
		transform.scale.set(30.0, 30.0, 30.0);
		transform.rotation.x = 3.14 / 2.0;
		transform.position.y = -3.25;


		root.add_child(
			NodeData::new("plane2")
				.set_transform(transform)
				.set_geometry(geometry_generators::plane_buffer_geometry(
					1.0, 1.0, 1, 1,
				).to_shared())
				.set_material(Material::new_basic(Vector4::new(0.8, 0.8, 0.8, 1.0)).to_shared())
				.to_shared()
		);
	}


	{
		let node = load_gltf(PathBuf::from("models/girl_speedsculpt/scene.gltf"), "girl").unwrap();
		root.add_child(node.clone());

		let mut node_data = node.lock();
		node_data.transform.position.y += 2.2;
		node_data.transform.position.x -= 2.0;
		node_data.transform.scale.set_scalar(0.4);
	}


	{
		root.add_child(load_gltf(PathBuf::from("models/Duck.glb"), "Duck").unwrap());
	}


	{
		let mut transform = Transform::default();
		transform.rotation.x = -3.14 / 180.0 * 45.0;
		transform.rotation.z = -3.14 / 180.0 * 45.0;

		let color = Vector3::new_one();
		let material_light = Material::new_basic(Vector4::new(color.x, color.y, color.z, 5.0)).to_shared();
		let mut light = Light::new(color.clone(), 5.0);
		light.light_type = LightType::Directional;

		root.add_child(
			NodeData::new("light")
				.set_transform(transform)
				.set_material(material_light.clone())
				.set_geometry(geom_light.clone())
				.set_light(light.clone())
				.to_shared()
		);
	}

	let (mut render_system, event_loop) = render::open_gl::system_render::RenderSystem::build(true, true, true);

	render_system.clear_color = Some(Vector4::new(0.0, 0.1, 0.1, 1.0));

	render_system.windowed_context.window().set_resizable(true);

	let hidpi_factor = render_system.windowed_context.window().scale_factor();
	let mut window_state = WindowState::default();



	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;

		match event {
			Event::WindowEvent {event, .. } => {
				match event {
					WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

					WindowEvent::Resized(physical_scale) => {
						let logical_size = physical_scale.to_logical::<u32>(hidpi_factor);

						window_state.window_size.0 = logical_size.width;
						window_state.window_size.1 = logical_size.height;

						render_system.windowed_context.resize(physical_scale);

						gl_call!({
							gl::Viewport(
								0,
								0,
								(logical_size.width as f64 * hidpi_factor) as i32,
								(logical_size.height as f64 * hidpi_factor) as i32,
							);
						});
						println!("logical_size: {:?}, hidpi_factor: {:?}", logical_size, hidpi_factor);
					}

					WindowEvent::MouseWheel{ delta, .. } => {
						match delta {
							MouseScrollDelta::LineDelta(_, y) => {
								if y > 0.0 {
									radius -= zoom_speed
								} else {
									radius += zoom_speed
								};
							}
							MouseScrollDelta::PixelDelta(_) => {}
						}
					}

					WindowEvent::CursorMoved{ position: pos, .. } => {
						window_state.pointer_pos = pos
							.to_logical::<f64>(hidpi_factor)
							.into();
					}
					_ => (),
				}
			},
			Event::Resumed => {
				println!("LOLOLOLOLO 12");
			},
			Event::RedrawRequested(_) => {
				println!("LOLOLOLOLO");

				{
					let mut node_data = camera.lock();
					{
						let transform_camera = &mut node_data.transform;
						let x_prog = window_state.pointer_pos.0 / window_state.window_size.0 as f64;
						let y_prog = window_state.pointer_pos.1 / window_state.window_size.1 as f64;
						transform_camera.position.z = ((x_prog * (PI_f64 * 2.0)).sin() * radius) as f32;
						transform_camera.position.x = ((x_prog * (PI_f64 * 2.0)).cos() * radius) as f32;
						transform_camera.position.y = ((y_prog * radius - radius / 2.0) * -2.0) as f32;
						transform_camera.look_at(&center, &up);
					}

					if let Some(camera) = &mut node_data.camera {
						let aspect = window_state.window_size.0 / window_state.window_size.1;
						camera.aspect = aspect as f32;
						camera.update_projection_matrix();
					}
				}


				root.update_transform(false);
				render_system.set_frame_buffer(Some(frame_buffer.clone()));
				render_system.render(&camera, &root);

				buffer_plane.update_transform(false);
				render_system.set_frame_buffer(None);
				render_system.render(&camera, &buffer_plane);
			}
			_ => (),
		}
	});
}
