use crate::actions::character_action::CharacterAction;
use crate::connection::Connection;
use crate::context::Context;
use crate::transactions::forward::forward;

pub mod forward;

pub fn call_transaction_with(
    conn: &Connection,
    context: &mut Context,
    action: CharacterAction,
) -> Result<(), String> {
    match action {
        CharacterAction::Forward(payload) => {
            forward(conn, context, &payload).and_then(|mutations| {
                context.mark_mutations(mutations);
                Ok(())
            })
        }
        _ => Err(format!("Action {:?} is not implemented", action)),
    }
}
