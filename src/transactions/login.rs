use crate::actions::system_action::SystemLoginPayload;
use crate::connection::Connection;
use crate::context::Context;
use crate::models::{character::*, Entity};
use crate::utils::generate_random_name;
use chrono::Utc;
use std::sync::Arc;

pub fn login(
    conn: &Connection,
    context: &mut Context,
    payload: &SystemLoginPayload,
) -> Result<Vec<i64>, String> {
    let updated = Vec::new();
    if let Some(character) = context.get_character_from_connection(conn) {
        Err(format!(
            "{:?} has already been associated to character {:?}",
            conn.addr, character
        ))
    } else {
        if let Some(character_id) = payload.character_id {
            if context
                .ip_to_character_id
                .iter()
                .any(|(_, cid)| *cid == character_id)
            {
                Err(format!(
                    "Character ID:{:?} has already been associated to someone",
                    character_id
                ))
            } else {
                if context
                    .characters
                    .iter()
                    .any(|(_, local)| local.model.id == character_id)
                {
                    context
                        .ip_to_character_id
                        .insert(conn.addr.clone(), character_id);
                    Ok(updated)
                } else {
                    Err(format!("Character ID:{:?} is not found", character_id))
                }
            }
        } else {
            // generate new character
            let character = Character {
                id: Utc::now().timestamp(),
                name: generate_random_name(7),
                max_hp: 100,
                max_appetite: 8000,
            };
            let mut character_local = CharacterLocal::from(character);
            character_local.angle.write(0.0);
            context
                .ip_to_character_id
                .insert(conn.addr.clone(), character_local.model.id);
            context.insert_entity(Entity::Character(Arc::new(character_local)));
            Ok(updated)
        }
    }
}
