use crate::counter::get_count;
use crate::init_field;
use crate::models::field::Field;
use crate::schema::characters;
use crate::utils::generate_random_name;
use rand::Rng;

#[derive(Queryable, Clone, Debug)]
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

#[derive(Debug)]
pub struct CharacterLocal {
    pub entity_id: u64,
    pub model: Character,

    pub hp: Field<i32>,
    pub appetite: Field<i32>,
    pub x: Field<f32>,
    pub y: Field<f32>,
    pub angle: Field<f32>,
    pub is_dead: Field<bool>,
    pub sleep_state: Field<Sleeping>,
    pub items: Field<Vec<u64>>,
    pub attack_charge: Field<f32>,
    pub item_charge: Field<f32>,
}

#[derive(Debug, Clone)]
pub struct Sleeping {
    pub state: SleepingState,
    pub depth: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SleepingState {
    GettingUp,
    Sleeping,
    Idle,
}

pub const MAX_SLEEP_AMOUNT: i32 = 60/*[sec]*/ * 90/*[min]*/;
pub const MIN_SLEEP_AMOUNT: i32 = 0;

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
        let sleep = Sleeping {
            state: SleepingState::GettingUp,
            depth: 0,
        };
        Self {
            entity_id: get_count(),
            model: model.clone(),
            hp: init_field!(model.max_hp),
            appetite: init_field!(model.max_appetite),
            x: init_field!(0.),
            y: init_field!(0.),
            angle: init_field!(rng.gen_range(0., 2. * std::f64::consts::PI as f32)),
            is_dead: init_field!(false),
            sleep_state: init_field!(sleep),
            items: init_field!(Vec::new()),
            attack_charge: init_field!(0.),
            item_charge: init_field!(0.),
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
