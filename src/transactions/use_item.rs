use crate::actions::character_action::CharacterUsePayload;
use crate::actions::character_action::CharacterGetFullPayload;
use crate::connection::Connection;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::dispatchers::item_dispatcher::ItemDispatcher
use crate::models::item::*;
use crate::err;

pub fn use_item(
    conn: &Connection,
    context: &mut Context,
    payload: &CharacterUsePayload,
) -> Result<Vec<i64>, String> {
    let mut updated = Vec::new();
    context
        .get_character_from_connection(conn)
        .and_then(|character| {
                let item_id = character.items.read().get(payload.item_index)
                    .and_then(|item_id|{
                        context.items.get(item_id)
                        .and_then(|local| {
                            match local.model.item_type {
                                ItemType::Unknown => unimplemented!()
                                ItemType::Food => {
                                    let get_full_payload = CharacterPushedPayload { local.model.amount };
                                    CharacterDispatcher::effect_get_full(&character, &get_full_payload)
                                        .and_then(|_| {
                                            updated.push(character.entity_id);
                                            ItemDispatcher::effect_spend(local);
                                                .and_then(|_| {
                                                    updated.push(local.entity_id);
                                                    Ok(())
                                                })
                                                .unwrap_or_else(|e| err!("{:?}", e));
                                        })
                                        .unwrap_or_else(|e| err!("{:?}", e));
                                }
                                ItemType::Weapon => unimplemented!()
                            }
                        })
                    })
                
            Ok(())
        })?;
    Ok(updated)
}
