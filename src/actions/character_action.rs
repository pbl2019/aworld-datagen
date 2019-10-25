#[derive(Debug)]
pub enum CharacterAction {
    Sleep { amount: i32 },
    Rest { amount: i32 },
    Idle { amount: i32 },
    GetUp { amount: i32 },
}

#[derive(Debug)]
pub enum CharacterEffect {
    Damage { amount: i32 },
    Heal { amount: i32 },
    WakeUp { amount: i32 }
    WokenUp { amount: i32 }
}
