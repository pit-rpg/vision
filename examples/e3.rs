extern crate uuid;
#[macro_use] extern crate project;


use std::f64::consts::PI as PI_f64;
use std::path::Path;

use project::{
	specs::*,
	glutin::{MouseScrollDelta},
	glutin,
	render,
	math::{Vector3, Vector},
	core::{
		SharedGeometry,
		PerspectiveCamera,
		Transform,
		SharedTexture2D,
		Material,
		SharedMaterials,
		create_world,
		ShaderProgram,
		SystemTransform,
		BufferType,
		UniformName,
		EntityRelations,
	},
	helpers::{load_obj},
};


#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WindowState {
	pub pointer_pos: (f64, f64),
	pub pointer_pressed: (bool, bool, bool),
	pub pointer_wheel: f32,
	pub window_size: (f64, f64),
}


fn main(){

	let mut world = create_world();
	let mut render_system = render::open_gl::system_render::RenderSystem::new(&mut world, true, true, true);
	let mut system_transform = SystemTransform::new();

	let up = Vector3::new(0.0, 1.0, 0.0);
	let center = Vector3::new_zero();
	let mut radius = 10.0;
	let zoom_speed = 0.5;


	let mut running = true;


	let mut camera = PerspectiveCamera::new();
	let mut transform_camera = Transform::default();
	transform_camera.position.z = 6.0;
	camera.view.enabled = false;

	let mut transform_light = Transform::default();
	transform_light.scale.set(0.2, 0.2, 0.2);


	let root = world
		.create_entity()
		.build();

	let e_cam = world
		.create_entity()
		.with(transform_camera)
		.with(camera)
		.build();


	let path = Path::new("models/Predator.obj");
	let objects = load_obj(&path).expect("cant load file");

	let mut mat_cup_mat = Material::new_mat_cup();
	let mat_cup_texture = SharedTexture2D::new_from_path("images/mc4.jpg");
	mat_cup_mat.set_uniform(UniformName::MapColor, mat_cup_texture.clone());
	let shared_mat_cup_mat = SharedMaterials::new(mat_cup_mat);


	for mut object in objects {

		if !object.has_attribute(BufferType::Normal) {
			object.generate_normals();
		}

		let geom = SharedGeometry::new(object);

		let transform = Transform::default();

		let entity = world
			.create_entity()
			.with(transform)
			.with(geom)
			.with(shared_mat_cup_mat.clone())
			.build();
		world.add_child(root, entity);
	}


	render_system.camera = Some(e_cam);
	render_system.windowed_context.window().set_resizable(true);
	let hidpi_factor = render_system.windowed_context.window().get_hidpi_factor();
	let mut window_state = WindowState::default();

	while running {

		{
			let windowed_context = &render_system.windowed_context;
			use self::glutin::WindowEvent::*;

			render_system.events_loop.poll_events(|event| {
				match event {
					glutin::Event::WindowEvent{ event, .. } => match event {
						glutin::WindowEvent::CloseRequested => running = false,
						glutin::WindowEvent::Resized(logical_size) => {
							println!("{:?}", logical_size);
							window_state.window_size.0 = logical_size.width;
							window_state.window_size.1 = logical_size.height;

							gl_call!({
								gl::Viewport(0,0, logical_size.width as i32, logical_size.height as i32);
							});
						},
						glutin::WindowEvent::MouseWheel{ delta, .. } => {
							match delta {
								MouseScrollDelta::LineDelta(_, y) => {
									if y > 0.0 { radius -= zoom_speed } else {radius += zoom_speed};
								}
								MouseScrollDelta::PixelDelta(_) => {}
							}
						}
						CursorMoved { position: pos, .. } =>{
							window_state.pointer_pos = pos
								.to_physical(windowed_context.window().get_hidpi_factor())
								.to_logical(hidpi_factor)
								.into();
						}
						_ => ()
					},
					_ => ()
				}
			});
		}


		{
			let mut transform_store = world.write_storage::<Transform>();
			let mut cam_store = world.write_storage::<PerspectiveCamera>();

			{
				let transform_camera = transform_store.get_mut(e_cam).unwrap();
				let aspect = window_state.window_size.0/window_state.window_size.1;

				let  camera = cam_store.get_mut(e_cam).unwrap();
				camera.aspect = aspect as f32;
				camera.update_projection_matrix();

				let x_prog = window_state.pointer_pos.0 / window_state.window_size.0;
				let y_prog = window_state.pointer_pos.1 / window_state.window_size.1;
				transform_camera.position.z = ( (x_prog * (PI_f64*2.0)).sin() * radius ) as f32;
				transform_camera.position.x = ( (x_prog * (PI_f64*2.0)).cos() * radius ) as f32;;
				transform_camera.position.y = (( y_prog * radius - radius/2.0) * -2.0) as f32;
				transform_camera.look_at(&center, &up);
			}
		}


		system_transform.run_now(&world);
		render_system.run(&mut world, root);
	}
}
