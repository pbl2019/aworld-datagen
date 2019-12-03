#[derive(Debug)]
pub enum CharacterAction {
    Forward(CharacterForwardPayload),
    Backward(CharacterBackwardPayload),
    TurnLeft(CharacterTurnLeftPayload),
    TurnRight(CharacterTurnRightPayload),
    Attack,
    Pickup,
    Use(CharacterUsePayload),
    Defence,
    Avoid,
    Sleep,
    Rest,
    Idle,
    GetUp,
}

#[derive(Debug)]
pub enum CharacterEffect {
    Pushed(CharacterPushedPayload),
    Damage(CharacterDamagedPayload),
    Dead,
    Recovery(CharacterRecoveryPayload),
    WakeUp,
    Disturb,
}

#[derive(Debug)]
pub struct CharacterForwardPayload {
    pub speed: f32,
}

#[derive(Debug)]
pub struct CharacterBackwardPayload {
    pub speed: f32,
}

#[derive(Debug)]
pub struct CharacterTurnLeftPayload {
    pub angle: f32,
}

#[derive(Debug)]
pub struct CharacterTurnRightPayload {
    pub angle: f32,
}

#[derive(Debug)]
pub struct CharacterUsePayload {
    pub item_index: i64,
}

#[derive(Debug)]
pub struct CharacterPushedPayload {
    pub angle: f32,
    pub speed: f32,
}

#[derive(Debug)]
pub struct CharacterDamagedPayload {
    pub amount: i32,
}

#[derive(Debug)]
pub struct CharacterRecoveryPayload {
    pub depth: i32,
}
