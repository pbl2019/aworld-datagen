use crate::connection::Connection;
use crate::context::Context;
use crate::models::terrain::*;
use crate::models::ObjectId;

pub fn pickup(
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
            let mut items = character.items.read();
            if let Some(obstacle) = context.raycast(x, y, angle, 1.0) {
                if let Obstacle::Object(object_id) = obstacle {
                    if let ObjectId::Item(item_id) = object_id {
                        items.push(item_id);
                        character.items.write(items);
                        updated.push(character.entity_id);
                    }
                }
            }
            Ok(())
        })?;
    Ok(updated)
}