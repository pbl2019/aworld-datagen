#[derive(Debug)]
pub enum CharacterAction {
    Attack,
    Use {itemId: String},
    Defence,
    Avoid,
  
    Sleep,
    Rest,
    Idle,
    GetUp,
}

#[derive(Debug)]
pub enum CharacterEffect {
    Damage { amount: i32 },
    Dead,

    Recovery { amount: i32 },
    WakeUp
    Disturb
}
