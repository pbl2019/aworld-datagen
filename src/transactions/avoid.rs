use crate::connection::Connection;
use crate::context::Context;
use crate::dispatchers::character_dispatcher::CharacterDispatcher;
use crate::err;

pub fn avoid(
    conn: &Connection,
    context: &mut Context,
) -> Result<Vec<i64>, String> {
    let mut updated = Vec::new();
    context
        .get_character_from_connection(conn)
        .and_then(|character| {
            CharacterDispatcher::action_avoid(&character)
                .and_then(|_| {
                    updated.push(character.entity_id);
                    Ok(())
                })
                .unwrap_or_else(|e| err!("{:?}", e));
            Ok(())
        })?;
    Ok(updated)
}
