extern crate uuid;
use self::uuid::Uuid;
use helpers::Nums;
use math::{Vector, Vector2, Vector3, Vector4};
use std::vec::Vec;
use std::fmt;
use std::sync::{Arc,Mutex, LockResult, MutexGuard};
use super::{Box3};


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum BufferType {
	Vector3(Vec<Vector3<f32>>),
	Vector4(Vec<Vector4<f32>>),
	Vector2(Vec<Vector2<f32>>),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BufferAttribute {
	pub data: BufferType,
	pub name: String,
	pub dynamic: bool,
	pub normalized: bool,
	pub version: usize,
}

#[allow(dead_code)]
impl BufferAttribute {
	pub fn count(&self) -> usize {
		let l = self.len();
		l / self.item_size()
	}

	pub fn item_size(&self) -> usize {
		match self.data {
			BufferType::Vector3(_) => 3,
			BufferType::Vector4(_) => 4,
			BufferType::Vector2(_) => 2,
		}
	}

	pub fn len(&self) -> usize {
		match &self.data {
			&BufferType::Vector4(ref a) => a.len(),
			&BufferType::Vector3(ref a) => a.len(),
			&BufferType::Vector2(ref a) => a.len(),
		}
	}

	pub fn set_normalized(&mut self, normalized: bool) -> &mut Self {
		self.normalized = normalized;
		self
	}

	pub fn set_version(&mut self, version: usize) -> &mut Self {
		self.version = version;
		self
	}

	pub fn set_dynamic(&mut self, dynamic: bool) -> &mut Self {
		self.dynamic = dynamic;
		self
	}
}

#[allow(dead_code)]
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct BufferGroup {
	pub start: usize,
	pub material_index: usize,
	pub count: usize,
	pub name: String,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct BufferGeometry {
	pub uuid: Uuid,
	pub name: String,
	pub groups: Vec<BufferGroup>,
	pub indices: Option<Vec<i32>>,
	pub attributes: Vec<BufferAttribute>,
	callbacks: Vec<fn(&mut BufferGeometry)>,
}


impl fmt::Debug for BufferGeometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=====================
BufferGeometry: {}
uuid: {}
groups: {:?}
callbacks: {}
indices: {:?}
attributes: {:?}
=====================",
		self.name,
		self.uuid,
		self.groups,
		self.callbacks.len(),
		self.indices,
		self.attributes,
		)
    }
}




extern crate specs;
use self::specs::{Component, VecStorage};


#[allow(dead_code)]
impl BufferGeometry {
	pub fn new() -> Self {
		Self {
			attributes: Vec::new(),
			groups: Vec::new(),
			indices: None,
			uuid: Uuid::new_v4(),
			callbacks: Vec::new(),
			name: "".to_string(),
		}
	}

	pub fn set_indices(&mut self, indices: Vec<i32>) -> &mut Self {
		self.indices = Some(indices);
		self
	}

	pub fn gen_indices(&mut self) -> Result<(), &str> {
		let mut len = 0;

		match self.get_attribute("positions") {
			None => {
				return Err("BufferGeometry: cant find positions");
			}
			Some(positions) => {
				len = positions.len();
			}
		};

		let indices = (0..len as i32).collect();

		self.set_indices(indices);

		Ok(())
	}

	pub fn add_buffer_attribute(
		&mut self,
		buffer_attribute: BufferAttribute,
	) -> &mut BufferAttribute {
		if self.attributes.len() > 0 {
			let len = buffer_attribute.len();
			let prev_len = self.attributes[0].len();
			if len != prev_len {
				panic!(
					"BufferGeometry: different buffer length {}:{}, {}:{}",
					buffer_attribute.name, len, self.attributes[0].name, prev_len
				);
			}
		}

		self.attributes.push(buffer_attribute);

		let i = self.attributes.len() - 1;
		&mut self.attributes[i]
	}

	pub fn create_buffer_attribute(
		&mut self,
		name: String,
		data: BufferType,
	) -> &mut BufferAttribute {
		let buffer_attribute = BufferAttribute {
			name,
			data,
			normalized: false,
			dynamic: false,
			version: 0,
		};

		self.add_buffer_attribute(buffer_attribute)
	}

	pub fn on_drop(&mut self, cb: fn(&mut BufferGeometry)) {
		self.callbacks.push(cb);
	}

	pub fn get_attribute(&self, name: &str) -> Option<&BufferAttribute> {
		self.attributes.iter().find(|e| e.name == name)
	}

	pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut BufferAttribute> {
		self.attributes.iter_mut().find(|e| e.name == name)
	}

	fn _compute_face_normals<T: Nums>(
		&self,
		positions: &Vec<Vector3<T>>,
		indices: &Vec<i32>,
	) -> Vec<Vector3<T>> {
		let len = indices.len() / 3;
		let mut normals = Vec::with_capacity(positions.len());

		for i in 0..len {
			let a = positions
				.get(*(indices.get(i * 3).unwrap()) as usize)
				.unwrap();
			let b = positions
				.get(*(indices.get(i * 3 + 1).unwrap()) as usize)
				.unwrap();
			let c = positions
				.get(*(indices.get(i * 3 + 2).unwrap()) as usize)
				.unwrap();

			let mut cb = c - b;
			let ab = a - b;
			cb.cross(&ab);
			cb.normalize();
			normals.push(cb)
		}
		normals
	}

	fn _compute_vertex_normals<T: Nums>(
		&self,
		face_normals: &Vec<Vector3<T>>,
		indices: &Vec<i32>,
	) -> Vec<Vector3<T>> {
		// let vertex_normals = self.get_attribute("positions").unwrap();
		// let indices = self.indices.as_ref().unwrap();

		// match vertex_normals.data {
		// 	BufferType::Vector3f32(ref normals) => {

		// 		for i in 0..(indices.len()/3) {
		// 			let normal = face_normals[i];
		// 			normals[i].copy(normal);
		// 		}

		// 	},
		// 	// BufferType::Vector3f64(normals) => {

		// 	// },
		// }

		unimplemented!()
	}

	pub fn compute_face_normals(&mut self) -> Option<&BufferAttribute> {
		let mut normals32 = None;
		// let mut normals64 = None;

		match self.get_attribute("positions") {
			None => return None,
			Some(attribute) => {
				match &attribute.data {
					&BufferType::Vector3(ref data) => {
						let mut normals =
							self._compute_face_normals(data, &self.indices.as_ref().unwrap());
						normals32 = Some(normals);
					}
					// &BufferType::Vector3f64(ref data) => {
					// 	let mut normals = self._compute_face_normals(data, &self.indices.as_ref().unwrap() );
					// 	normals64 = Some(normals);
					// },
					_ => return None,
				}
			}
		};

		match normals32 {
			Some(normals) => {
				let buffer_attribute = self.create_buffer_attribute(
					"face_normals".to_string(),
					BufferType::Vector3(normals),
				);
				return Some(buffer_attribute);
			}
			_ => {}
		}

		// match normals64 {
		// 	Some(normals) => {
		// 		let buffer_attribute = self.create_buffer_attribute("face_normals".to_string(), BufferType::Vector3f64(normals), 3);
		// 		return Some(buffer_attribute);
		// 	},
		// 	_=>{}
		// }

		None
	}

	pub fn compute_vertex_normals(&mut self) -> Option<&BufferAttribute> {
		let mut normals32 = None;

		{
			let face_normals = self.get_attribute("face_normals").unwrap();

			match face_normals.data {
				BufferType::Vector3(ref normals) => {
					let mut normals =
						self._compute_vertex_normals(normals, &self.indices.as_ref().unwrap());
					normals32 = Some(normals);
				}
				_ => return None,
			}
		}

		match normals32 {
			Some(normals) => {
				let buffer_attribute = self.create_buffer_attribute(
					"vertex_normals".to_string(),
					BufferType::Vector3(normals),
				);
				return Some(buffer_attribute);
			}
			_ => {}
		}

		None
	}

	pub fn duplicate(&self) -> Self {
		let mut data = self.clone();
		data.uuid = Uuid::new_v4();
		data
	}

	pub fn create_box3 (&self) -> Option<Box3<f32>> {
		if let Some(attr) = self.get_attribute("positions") {
			if let BufferType::Vector3(positions) = &attr.data {
				let mut b = Box3::new_empty();
				b.set_from_array(&positions[..]);
				return Some(b);
			}
			return None;
		}
		None
	}

	pub fn scale_positions_by_vec(&mut self, v: &Vector3<f32>) -> Option<()> {
		if let Some(attr) = self.get_attribute_mut("positions") {
			if let BufferType::Vector3(positions) = &mut attr.data {
				positions
					.iter_mut()
					.for_each(|e| {
						e.multiply(v);
					});
				return Some(());
			}
			return None;
		}
		None
	}
}

impl Drop for BufferGeometry {
	fn drop(&mut self) {
		while self.callbacks.len() > 0 {
			let cb = self.callbacks.pop().unwrap();
			cb(self);
		}
	}
}


// pub type Geometry = Arc<Mutex<BufferGeometry>>;

#[derive(Clone)]
pub struct SharedGeometry (Arc<Mutex<BufferGeometry>>);

impl SharedGeometry {
	pub fn new(g: BufferGeometry) -> Self {
		SharedGeometry(Arc::new(Mutex::new(g)))
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<BufferGeometry>> {
		self.0.lock()
	}
}


impl Component for SharedGeometry {
	type Storage = VecStorage<Self>;
}
// impl Component for BufferGeometry {
// 	type Storage = VecStorage<Self>;
// }
