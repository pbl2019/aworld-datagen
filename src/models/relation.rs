use crate::counter::get_count;
use crate::init_field;
use crate::models::field::Field;
use crate::schema::relations;
use rand::Rng;

#[derive(Queryable, Clone, Debug)]
pub struct Relation {
    pub id: i64,
    pub character_id: i64,
    pub target_id: i64,
    pub factor: f64,
}

#[derive(Insertable)]
#[table_name = "relations"]
pub struct NewRelation {
    pub character_id: i64,
    pub target_id: i64,
    pub factor: f64,
}

#[derive(Debug)]
pub struct RelationLocal {
    pub entity_id: u64,
    pub model: Relation,

    pub factor: Field<f64>,
}

impl NewRelation {
    pub fn new_random(character_id: i64, target_id: i64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            character_id,
            target_id,
            factor: rng.gen_range(-1.0, 1.0),
        }
    }
}

impl std::convert::From<Relation> for RelationLocal {
    fn from(model: Relation) -> Self {
        Self {
            entity_id: get_count(),
            model: model.clone(),
            factor: init_field!(model.factor),
        }
    }
}

#[test]
fn create_relation() {
    let new_relation = NewRelation::new_random(0, 1);
    assert!(new_relation.character_id >= 0);
    assert!(new_relation.target_id >= 0);
    assert!(-1.0 <= new_relation.factor && new_relation.factor <= 1.0);
}
