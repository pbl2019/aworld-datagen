use crate::actions::character_action::CharacterEffect;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::err;
use crate::models::character::*;

pub fn charge_down(context: &mut Context) -> Result<Vec<u64>, String> {
    let mut updated = Vec::new();
    for (entity_id, local) in &context.characters {
        let attack_charge = local.attack_charge.read() - 1.0;
        if attack_charge >= 0.0 {
            local.attack_charge.write(attack_charge);
            updated.push(*entity_id);
        }
        let item_charge = local.item_charge.read() - 1.0;
        if item_charge >= 0.0 {
            local.item_charge.write(item_charge);
            updated.push(*entity_id);
        }
    }
    Ok(updated)
}
