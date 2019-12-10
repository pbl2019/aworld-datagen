use crate::actions::character_action::CharacterGetFullPayload;
use crate::actions::character_action::CharacterUsePayload;
use crate::connection::Connection;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::dispatchers::item_dispatcher::ItemDispatcher;
use crate::models::item::*;
use crate::{dbg, err};

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
            let item_id = items
                .get(payload.item_index as usize)
                .ok_or_else(|| {
                    err!(
                        "item_index {} is out of length {}",
                        payload.item_index,
                        items_len
                    )
                })
                .and_then(|item_id| {
                    if let Some(local) = context.items.get(item_id) {
                        match local.model.item_type {
                            ItemType::Unknown => {
                                Err(dbg!("Item {} is not able to use", local.model.name))
                            }
                            ItemType::Food => {
                                let get_full_payload = CharacterGetFullPayload {
                                    amount: local.model.amount,
                                };
                                CharacterDispatcher::effect_get_full(&character, &get_full_payload)
                                    .and_then(|_| {
                                        updated.push(character.entity_id);
                                        ItemDispatcher::effect_spend(local).and_then(|_| {
                                            updated.push(local.entity_id);
                                            Ok(())
                                        })
                                    })
                                    .or_else(|e| Err(err!("{:?}", e)))
                            }
                            ItemType::Weapon => unimplemented!(),
                        }
                    } else {
                        Err(err!("Not found item<{}>", item_id))
                    }
                });
            Ok(())
        })?;
    Ok(updated)
}
