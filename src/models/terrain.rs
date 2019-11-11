use crate::init_field;
use crate::models::field::Field;
use crate::models::ObjectId;
use crate::schema::terrains;
use base64;
use chrono::Utc;
use rand::Rng;

#[derive(Queryable, Clone, Debug)]
pub struct Terrain {
    pub id: i64,
    pub content: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Insertable)]
#[table_name = "terrains"]
pub struct NewTerrain {
    pub content: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone)]
pub enum Obstacle {
    Object(ObjectId),
    Terrain(TerrainInfo),
}

#[derive(Clone, Debug)]
pub enum TerrainInfo {
    Floor = 0,
    Wall = 1,
}

#[derive(Debug)]
pub struct TerrainLocal {
    pub entity_id: i64,
    pub model: Terrain,

    pub raw: Field<Vec<u8>>,
    pub entities: Field<Vec<ObjectId>>,
}

impl TerrainLocal {
    pub fn raycast(&self, _x: f32, _y: f32, _angle: f32, _distance: f32) -> Option<Obstacle> {
        unimplemented!()
    }
}

impl std::default::Default for NewTerrain {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(50, 100);
        let height = rng.gen_range(50, 100);
        let mut raw = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            raw.push(rng.gen_range(0, 1 + 1));
        }
        Self {
            content: base64::encode(&raw),
            width: width as i32,
            height: height as i32,
        }
    }
}

impl std::convert::From<Terrain> for TerrainLocal {
    fn from(model: Terrain) -> Self {
        let raw = base64::decode(&model.content).unwrap();
        Self {
            entity_id: Utc::now().timestamp(),
            model: model.clone(),

            raw: init_field!(raw),
            entities: init_field!(Vec::new()),
        }
    }
}

#[test]
fn create_character() {
    let new_terrain = NewTerrain::default();
    assert!(new_terrain.content.len() > 0);
    let raw = base64::decode(&new_terrain.content).unwrap();
    assert_eq!(raw.len(), (new_terrain.width * new_terrain.height) as usize);
    for i in 0..new_terrain.height {
        for j in 0..new_terrain.width {
            print!("{}", raw[(i * new_terrain.width + j) as usize])
        }
        print!("\n");
    }
}
