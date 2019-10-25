#[derive(Debug)]
pub enum CharacterEffect {
    Damage { amount: i32 },
    Dead {},
}

pub enum CharacterAction {
    Attack {},
    Use {itemId: String},
    Defence {},
    Avoid{},
}