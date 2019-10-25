#[derive(Debug)]
pub enum CharacterAction {
    Sleep,
    Rest,
    Idle,
    GetUp,
}

#[derive(Debug)]
pub enum CharacterEffect {
    Damage { amount: i32 },
    Recovery { amount: i32 },
    WakeUp
    Disturb
}
