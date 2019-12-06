use crate::actions::{character_action::CharacterAction, system_action::SystemAction, Action};
use crate::connection::Connection;
use crate::context::Context;
use crate::transactions::{
    attack::attack, forward::forward, login::login, pickup::pickup, turn_left::turn_left,
    turn_right::turn_right,
};

pub mod attack;
pub mod forward;
pub mod login;
pub mod pickup;
pub mod turn_left;
pub mod turn_right;

pub fn call_transaction_with(
    conn: &Connection,
    context: &mut Context,
    action: Action,
) -> Result<(), String> {
    match action {
        Action::Character(character_action) => match character_action {
            CharacterAction::Attack => attack(conn, context).and_then(|mutations| {
                context.mark_mutations(mutations);
                Ok(())
            }),
            CharacterAction::Forward(payload) => {
                forward(conn, context, &payload).and_then(|mutations| {
                    context.mark_mutations(mutations);
                    Ok(())
                })
            }
            CharacterAction::TurnLeft(payload) => {
                turn_left(conn, context, &payload).and_then(|mutations| {
                    context.mark_mutations(mutations);
                    Ok(())
                })
            }
            CharacterAction::TurnRight(payload) => {
                turn_right(conn, context, &payload).and_then(|mutations| {
                    context.mark_mutations(mutations);
                    Ok(())
                })
            }
            CharacterAction::Pickup => pickup(conn, context).and_then(|mutations| {
                context.mark_mutations(mutations);
                Ok(())
            }),
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
