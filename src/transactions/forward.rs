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
) -> Result<Vec<u64>, String> {
    let mut updated = Vec::new();
    context
        .get_character_from_connection(conn)
        .and_then(|character| {
            let width = context.terrain.model.width;
            let height = context.terrain.model.height;
            let x = character.x.read();
            let y = character.y.read();
            let angle = character.angle.read();
            let speed = payload.speed;
            let mut is_ignore_obstacle = true;
            if let Some(obstacle) = context.raycast(x, y, angle, speed) {
                match obstacle {
                    Obstacle::Object(object_id) => match object_id {
                        ObjectId::Character(character_id) => {
                            let pushee = context.characters.get(&character_id).unwrap();
                            let pushed_payload = CharacterPushedPayload { angle, speed };
                            is_ignore_obstacle = false;
                            CharacterDispatcher::effect_pushed(&pushee, &pushed_payload)
                                .and_then(|_| {
                                    updated.push(pushee.entity_id);
                                    Ok(())
                                })
                                .unwrap_or_else(|e| err!("{:?}", e));
                        }
                        ObjectId::Item(item_id) => {
                            dbg!("{} moved over an item<{}>", character.model.name, item_id);
                        }
                    },
                    Obstacle::Terrain(info) => {
                        if info == TerrainInfo::Wall {
                            is_ignore_obstacle = false;
                            // let fixed_x = x.floor().max(1.0).min((width - 1) as f32) - 0.50;
                            // let fixed_y = y.floor().max(1.0).min((height - 1) as f32) - 0.50;
                            let mut fixed_x = x + speed * (angle + std::f32::consts::PI).cos();
                            let mut fixed_y = y + speed * (angle * std::f32::consts::PI).sin();
                            if fixed_x.floor() as i32 != x.floor() as i32 {
                                fixed_x = x.floor().max(0.0).min((width - 1) as f32) + 0.5;
                            }
                            if fixed_y.floor() as i32 != y.floor() as i32 {
                                fixed_y = y.floor().max(0.0).min((height - 1) as f32) + 0.5;
                            }
                            println!("{}, {}", fixed_x, fixed_y);
                            let fix_speed = ((fixed_x - x).powi(2) + (fixed_y - y).powi(2)).sqrt();
                            let fix_angle = (fixed_y - y).atan2(fixed_x - x);
                            let pushed_payload = CharacterPushedPayload {
                                angle: fix_angle,
                                speed: fix_speed,
                            };
                            character.x.write(fixed_x);
                            character.y.write(fixed_y);
                            // CharacterDispatcher::effect_pushed(&character, &pushed_payload)
                            //     .and_then(|_| {
                            //         updated.push(character.entity_id);
                            //         Ok(())
                            //     })
                            //     .unwrap_or_else(|e| err!("{:?}", e));
                            dbg!("{} tackled to {:?}", character.model.name, info);
                        }
                    }
                }
            }
            if is_ignore_obstacle {
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
