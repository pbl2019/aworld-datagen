use crate::actions::character_action::*;
use crate::connection::Connection;
use crate::context::Context;
use crate::models::terrain::*;
use crate::models::ObjectId;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::err;


pub fn attack(
	conn: &Connection,
	context: &mut Context,
) -> Result<Vec<i64>, String> {
	let mut updated = Vec::new();
	context
		.get_character_from_connection(conn)
		.and_then(|character| {
			let x = character.x.read();
			let y = character.y.read();
			let angle = character.angle.read();
			
			if let Some(obstacle) = context.raycast(x, y, angle, 1.0) {
				match obstacle {
					Obstacle::Object(object_id) => match object_id {
						ObjectId::Character(character_id) => {
							let damagee = context.characters.get(&character_id).unwrap();
							let payload = CharacterDamagedPayload {amount: 10};
							print!("{:?}", payload);
							CharacterDispatcher::effect_damage(&damagee, &payload)
								.and_then(|_| {
									Ok(())
								})
								.unwrap_or_else(|e| err!("{:?}", e));
						}
						ObjectId::Item(_item_id) => unimplemented!(),
					},
					Obstacle::Terrain(info) => {
                        dbg!("{} attacked to {:?}", &character.model.name, info);
                    }
				}
			} else {
				CharacterDispatcher::action_attack(&character)
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
