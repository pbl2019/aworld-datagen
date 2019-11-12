use crate::actions::{character_action::CharacterAction, system_action::SystemAction, Action};
use crate::connection::Connection;
use crate::context::Context;
use crate::transactions::{forward::forward, login::login};

pub mod forward;
pub mod login;

pub fn call_transaction_with(
    conn: &Connection,
    context: &mut Context,
    action: Action,
) -> Result<(), String> {
    match action {
        Action::Character(character_action) => match character_action {
            CharacterAction::Forward(payload) => {
                forward(conn, context, &payload).and_then(|mutations| {
                    context.mark_mutations(mutations);
                    Ok(())
                })
            }
            _ => Err(format!("Action {:?} is not implemented", character_action)),
        },
        Action::System(system_action) => match system_action {
            SystemAction::Login(payload) => login(conn, context, &payload).and_then(|mutations| {
                context.mark_mutations(mutations);
                Ok(())
            }),
            // _ => Err(format!("Action {:?} is not implemented", action)),
        },
    }
}
