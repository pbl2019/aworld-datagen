#[derive(Debug)]
pub enum CharacterAction {
    Forward(CharacterForwardPayload),
    Backward(CharacterBackwardPayload),
    TurnLeft(CharacterTurnLeftPayload),
    TurnRight(CharacterTurnRightPayload),
    Attack,
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
    speed: f32,
}

#[derive(Debug)]
pub struct CharacterBackwardPayload {
    speed: f32,
}

#[derive(Debug)]
pub struct CharacterTurnLeftPayload {
    angle: f32,
}

#[derive(Debug)]
pub struct CharacterTurnRightPayload {
    angle: f32,
}

#[derive(Debug)]
pub struct CharacterUsePayload {
    item_id: String,
}

#[derive(Debug)]
pub struct CharacterPushedPayload {
    angle: f32,
    speed: f32,
}

#[derive(Debug)]
pub struct CharacterDamagedPayload {
    amount: i32,
}

#[derive(Debug)]
pub struct CharacterRecoveryPayload {
    amount: i32,
}
