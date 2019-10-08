use crate::schema::characters;

#[derive(Queryable)]
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

impl std::default::Default for NewCharacter {
    fn default() -> Self {
        Self {
            name: "foo".to_owned(),
            max_hp: 1234,
            max_appetite: 56789,
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
