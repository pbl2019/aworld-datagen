use crate::actions::character_action::{CharacterForwardPayload, CharacterPushedPayload};
use crate::connection::*;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::models::terrain::*;
use crate::models::ObjectId;
use crate::{dbg, err};

pub fn forward(
    conn: &Connection,
    context: &mut Context,
    payload: &CharacterForwardPayload,
) -> Result<Vec<i64>, String> {
    let mut updated = Vec::new();
    context
        .get_character_from_connection(conn)
        .and_then(|character| {
            let x = character.x.read();
            let y = character.y.read();
            let angle = character.angle.read();
            let speed = payload.speed;
            if let Some(obstacle) = context.raycast(x, y, angle, speed) {
                match obstacle {
                    Obstacle::Object(object_id) => match object_id {
                        ObjectId::Character(character_id) => {
                            let pushee = context.characters.get(&character_id).unwrap();
                            let pushed_payload = CharacterPushedPayload { angle, speed };
                            CharacterDispatcher::effect_pushed(&pushee, &pushed_payload)
                                .and_then(|_| {
                                    updated.push(pushee.entity_id);
                                    Ok(())
                                })
                                .unwrap_or_else(|e| err!("{:?}", e));
                        }
                        ObjectId::Item(_item_id) => unimplemented!(),
                    },
                    Obstacle::Terrain(info) => {
                        dbg!("{} tackled to {:?}", character.model.name, info);
                    }
                }
            } else {
                CharacterDispatcher::action_forward(&character, payload)
                    .and_then(|_| {
                        updated.push(character.entity_id);
                        Ok(())
                    })
                    .unwrap_or_else(|e| err!("{:?}", e));
            }
            Ok(())
        })?;
    Ok(updated)
}
