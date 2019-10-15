use crate::schema::items;
use rand::{thread_rng, Rng};

#[derive(Queryable)]
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

impl NewRelation {
    pub fn new_random(character_id: i64, target_id: i64) -> Self {
        Self {
            character_id,
            target_id,
            factor: rng.gen_range(-1.0, 1.0),
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
