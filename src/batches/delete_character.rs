use crate::actions::character_action::CharacterEffect;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::err;
use crate::models::character::*;

pub fn delete_character(context: &mut Context) -> Result<Vec<u64>, String> {
    let mut updated = Vec::new();
    context.characters.retain(|_, local|!local.is_dead.read());
    Ok(updated)
}