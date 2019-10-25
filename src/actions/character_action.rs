pub enum CharacterAction {
    Forward { speed: f32 },
    Back { speed: f32 },
    TurnLeft { angle: f32 },
    TurnRight { angle: f32 },
}

#[derive(Debug)]
pub enum CharacterEffect {
    Damage { amount: i32 },
}
