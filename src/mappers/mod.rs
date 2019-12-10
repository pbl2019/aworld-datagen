use crate::actions::{character_action::*, system_action::*, Action};
use crate::query::*;
use std::convert::TryFrom;

mod backward;
mod forward;
mod login;
mod turn_left;
mod turn_right;
mod use_item;

pub fn query_to_action(query: &Query) -> Result<Action, String> {
    match query.kind {
        QueryKind::Login => TryFrom::try_from(&query.payload)
            .and_then(|payload| Ok(Action::System(SystemAction::Login(payload)))),
        QueryKind::Forward => TryFrom::try_from(&query.payload)
            .and_then(|payload| Ok(Action::Character(CharacterAction::Forward(payload)))),
        QueryKind::Backward => {
            let payload = TryFrom::try_from(&query.payload).unwrap();
            Ok(Action::Character(CharacterAction::Backward(payload)))
        }
        QueryKind::Attack => Ok(Action::Character(CharacterAction::Attack)),
        QueryKind::TurnLeft => {
            let payload = TryFrom::try_from(&query.payload).unwrap();
            Ok(Action::Character(CharacterAction::TurnLeft(payload)))
        }
        QueryKind::TurnRight => {
            let payload = TryFrom::try_from(&query.payload).unwrap();
            Ok(Action::Character(CharacterAction::TurnRight(payload)))
        }
        QueryKind::Pickup => Ok(Action::Character(CharacterAction::Pickup)),
        QueryKind::UseItem => TryFrom::try_from(&query.payload)
            .and_then(|payload| Ok(Action::Character(CharacterAction::Use(payload)))),
        _ => Err(format!("Cannot convert query {:?} to action", query.kind)),
    }
}
