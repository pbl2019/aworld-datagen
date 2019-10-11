use crate::schema::terrains;
extern crate base64;
use base64::{encode, decode};

#[derive(Queryable)]
pub struct Terrain {
	pub id: i64,
	pub content: String,
	pub width: i32,
	pub height: i32,
}

#[derive(Insertable)]
#[table_name = "terrains"]
pub struct NewTerrain {
	pub content: String ,
	pub width: i32,
	pub height: i32,
}

impl std::default::Default for NewTerrain {
	fn default() -> Self {
		Self {
			content: encode(&vec![0u8; 10000]),
			width: 100,
			height: 100,
		}
	}
}

#[test]
fn create_character() {
	let new_terrain = NewTerrain::default();
	assert!(new_terrain.content.len() > 0);
}
