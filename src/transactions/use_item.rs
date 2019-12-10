use crate::actions::character_action::CharacterGetFullPayload;
use crate::actions::character_action::CharacterUsePayload;
use crate::connection::Connection;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::dispatchers::item_dispatcher::ItemDispatcher;
use crate::models::item::*;

pub fn use_item(
    conn: &Connection,
    context: &mut Context,
    payload: &CharacterUsePayload,
) -> Result<Vec<u64>, String> {
    let mut updated = Vec::new();
    context
        .get_character_from_connection(conn)
        .and_then(|character| {
            let items = character.items.read();
            let items_len = items.len();
            if let Some(item_id) = items.get(payload.item_index as usize) {
                if let Some(local) = context.items.get(item_id) {
                    match local.model.item_type {
                        ItemType::Unknown => {
                            Err(format!("Item {} is not able to use", local.model.name))
                        }
                        ItemType::Food => {
                            let get_full_payload = CharacterGetFullPayload {
                                amount: local.model.amount,
                            };
                            CharacterDispatcher::effect_get_full(&character, &get_full_payload)
                                .and_then(|_| {
                                    {
                                        let mut items = character.items.read().clone();
                                        items.remove(payload.item_index as usize);
                                        character.items.write(items);
                                    }
                                    updated.push(character.entity_id);
                                    ItemDispatcher::effect_spend(local).and_then(|_| {
                                        updated.push(local.entity_id);
                                        Ok(())
                                    })
                                })
                                .or_else(|e| Err(format!("{:?}", e)))
                        }
                        ItemType::Weapon => unimplemented!(),
                    }
                } else {
                    Err(format!("Not found item<{}>", item_id))
                }
            } else {
                Err(format!(
                    "item_index {} is out of length {}",
                    payload.item_index, items_len
                ))
            }
        })?;
    Ok(updated)
}
