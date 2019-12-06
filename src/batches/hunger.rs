use crate::actions::character_action::CharacterEffect;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::err;
use crate::models::character::*;

pub fn hunger(context: &mut Context) -> Result<Vec<u64>, String> {
    let mut updated = Vec::new();
    for (entity_id, local) in &context.characters {
        let appetite = local.appetite.read() - 1;
        local.appetite.write(appetite);
        if appetite <= 0 {
            let hp = local.hp.read() - 1;
            local.hp.write(hp);
            updated.push(*entity_id);
        }
    }
    Ok(updated)
}
