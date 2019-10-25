#[derive(Debug)]
pub enum CharacterAction {
    Forward(CharacterForwardPayload),
    Backward(CharacterBackwardPayload),
    TurnLeft(CharacterTurnLeftPayload),
    TurnRight(CharacterTurnRightPayload),

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
    Pushed(CharacterPushedPayload)

    Damage { amount: i32 },
    Dead,

    Recovery(CharacterRecoveryPayload),
    WakeUp
    Disturb
}

pub struct CharacterForwardPayload {
    speed: f32
}

pub struct CharacterBackwardPayload {
    speed: f32
}

pub struct CharacterTurnLeftPayload {
    angle: f32
}

pub struct CharacterTurnRightPayload {
    angle: f32
}

pub struct CharacterPushedPayload {
    angle: f32,
    speed: f32,
}

pub struct CharacterRecoveryPayload {
    amount: i32
}
