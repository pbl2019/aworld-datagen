use crate::schema::characters;
use crate::utils::generate_random_name;
use rand::{thread_rng, Rng};

#[derive(Queryable, Clone)]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub max_hp: i32,
    pub max_appetite: i32,
}

#[derive(Insertable)]
#[table_name = "characters"]
pub struct NewCharacter {
    pub name: String,
    pub max_hp: i32,
    pub max_appetite: i32,
}

pub struct CharacterLocal {
    pub hp: i32,
    pub appetite: i32,
    pub x: i32,
    pub y: i32,
    pub angle: f32,
    pub model: Character,
}

impl std::default::Default for NewCharacter {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            name: generate_random_name(0),
            max_hp: 8000 + rng.gen_range(0, 4000),
            max_appetite: 8000 + rng.gen_range(0, 4000),
        }
    }
}

impl std::convert::From<Character> for CharacterLocal {
    fn from(model: Character) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            model: model.clone(),
            hp: model.max_hp,
            appetite: model.max_appetite,
            x: 0,
            y: 0,
            angle: rng.gen_range(0., 2. * std::f64::consts::PI as f32),
        }
    }
}

#[test]
fn create_character() {
    let new_character = NewCharacter::default();
    assert!(new_character.name.len() > 0);
    assert!(new_character.max_hp > 0);
    assert!(new_character.max_appetite > 0);
}
