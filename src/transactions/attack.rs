use crate::actions::character_action::CharacterDamagedPayload;
use crate::connection::*;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::err;
use crate::models::terrain::*;
use crate::models::ObjectId;
use crate::environment::Environment;
use std::sync::{Arc, RwLock};

const FULL_CHARGE:f32 = 50.0;

pub fn attack(conn: &Connection, context: &mut Context) -> Result<Vec<u64>, String> {
    let mut updated = Vec::new();
    context
        .get_character_from_connection(conn)
        .and_then(|character| {
            let x = character.x.read();
            let y = character.y.read();
            let angle = character.angle.read();
            let attack_charge = character.attack_charge.read() + 1.0;
            if attack_charge >= FULL_CHARGE {
                if let Some(obstacle) = context.raycast(x, y, angle, 1.0) {
                    if let Obstacle::Object(object_id) = obstacle {
                        if let ObjectId::Character(character_id) = object_id {
                            let target = context.characters.get(&character_id).unwrap();
                            let target_clone = Arc::new(target.clone());
                            CharacterDispatcher::effect_damage(
                                target,
                                &CharacterDamagedPayload { amount: 10 },
                            )
                            .and_then(|_| {
                                if target_clone.is_dead.read() {
                                    let item_id = Environment::generate_meet(context, target_clone.x.read(), target_clone.y.read()).unwrap();
                                    updated.push(item_id);
                                }
                                Ok(())
                            })
                            .and_then(|_| {
                                character.attack_charge.write(0.0);
                                Ok(())
                            })
                            .and_then(|_| {
                                updated.push(character_id);
                                Ok(())
                            })
                            .unwrap_or_else(|e| err!("{:?}", e));
                        }
                    }
                }
            } else {
                character.attack_charge.write(attack_charge);
                updated.push(character.entity_id);
            }
            Ok(())
        })?;
    Ok(updated)
}
