use crate::schema::terrains;

#[derive(Queryable)]
pub struct Terrain {
	pub id: i64,
	pub content: String,
}

#[derive(Insertable)]
#[table_name = "terrains"]
pub struct NewTerrain {
	pub content: String,
}

impl std::default::Default for NewTerrain {
	fn default() -> Self {
		Self {
			content: "00000000000000000000000000".to_string(),
		}
	}
}

#[test]
fn create_character() {
	let new_terrain = NewTerrain::default();
	assert!(new_terrain.content.len() > 0);
}
