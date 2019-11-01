use crate::init_field;
use crate::models::field::Field;
use crate::schema::terrains;
use base64::{decode, encode};
use rand::Rng;

#[derive(Queryable, Clone)]
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

pub struct TerrainLocal {
    pub model: Terrain,
}

impl std::default::Default for NewTerrain {
    fn default() -> Self {
        let mut content = "".to_string();
        for i in 0..500 {
            content += "#";
        }
        let mut rng = rand::thread_rng();
        for i in 5..95 {
            content += "#####";
            for j in 5..95 {
                let a: i32 = rng.gen();
                if (a % 2 == 0) {
                    content += " ";
                } else {
                    content += "#";
                }
            }
            content += "#####";
        }
        for i in 0..500 {
            content += "#";
        }
        Self {
            //content: encode(&vec![0u8; 10000]),
            content: content,
            width: 100,
            height: 100,
        }
    }
}

impl std::convert::From<Terrain> for TerrainLocal {
    fn from(model: Terrain) -> Self {
        Self {
            model: model.clone(),
        }
    }
}

#[test]
fn create_character() {
    let new_terrain = NewTerrain::default();
    assert!(new_terrain.content.len() > 0);
    for i in 0..100 {
        for j in 0..100 {
            print!("{}", new_terrain.content.chars().nth(i * 100 + j).unwrap())
        }
        println!("")
    }
}
