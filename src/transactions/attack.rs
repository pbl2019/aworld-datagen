use crate::actions::character_action::CharacterDamagedPayload;
use crate::connection::*;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::err;
use crate::models::terrain::*;
use crate::models::ObjectId;

pub fn attack(conn: &Connection, context: &mut Context) -> Result<Vec<u64>, String> {
    let mut updated = Vec::new();
    context
        .get_character_from_connection(conn)
        .and_then(|character| {
            let x = character.x.read();
            let y = character.y.read();
            let angle = character.angle.read();
            if let Some(obstacle) = context.raycast(x, y, angle, 1.0) {
                if let Obstacle::Object(object_id) = obstacle {
                    if let ObjectId::Character(character_id) = object_id {
                        let target = context.characters.get(&character_id).unwrap();
                        CharacterDispatcher::effect_damage(
                            target,
                            &CharacterDamagedPayload { amount: 10 },
                        )
                        .and_then(|_| {
                            updated.push(character_id);
                            Ok(())
                        })
                        .unwrap_or_else(|e| err!("{:?}", e));
                    }
                }
            }
            Ok(())
        })?;
    Ok(updated)
}
