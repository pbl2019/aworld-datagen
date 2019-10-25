#[derive(Debug)]
pub enum CharacterEffect {
    Damage { amount: i32 },
    Die { amount : i32 },
}

pub enum CharacterAction {
    Attack { amount: i32 },
    Use { amount: i32 },
    Defence { amount: i32 },
    Avoidance { amount: i32 },
}