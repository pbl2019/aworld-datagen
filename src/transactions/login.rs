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
    let mut updated = Vec::new();
    match context.get_character_from_connection(conn) {
        Ok(character) => Err(format!(
            "{:?} has already been associated to character {:?}",
            conn.addr, character
        )),
        Err(_) => {
            if let Some(character_id) = payload.character_id {
                if context
                    .connection_to_character_id
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
                            .connection_to_character_id
                            .insert(conn.clone(), character_id);
                        let character = context.characters.get(&character_id).unwrap();
                        // updated.push(character.entity_id);
                        updated.extend_from_slice(&context.get_entity_ids());
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
                let character_local = CharacterLocal::from(character);
                character_local.angle.write(0.0);
                // updated.push(character_local.entity_id);
                updated.extend_from_slice(&context.get_entity_ids());
                context
                    .connection_to_character_id
                    .insert(conn.clone(), character_local.model.id);
                context.insert_entity(Entity::Character(Arc::new(character_local)));
                Ok(updated)
            }
        }
    }
}
