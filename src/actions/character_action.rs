pub enum CharacterAction {
    Forward { speed: f32 },
    Backward { speed: f32 },
    TurnLeft { angle: f32 },
    TurnRight { angle: f32 },
}

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
