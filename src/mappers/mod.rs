use crate::actions::character_action::*;
use crate::query::*;
use std::convert::TryFrom;

mod backward;
mod forward;
mod turn_left;
mod turn_right;

pub fn query_to_action(query: &Query) -> Result<CharacterAction, &'static str> {
    match query.kind {
        QueryKind::Forward => {
            let payload = TryFrom::try_from(&query.payload).unwrap();
            Ok(CharacterAction::Forward(payload))
        }
        QueryKind::Backward => {
            let payload = TryFrom::try_from(&query.payload).unwrap();
            Ok(CharacterAction::Backward(payload))
        }
        QueryKind::Attack => {
            Ok(CharacterAction::Attack)
        }
        QueryKind::TurnLeft => {
            let payload = TryFrom::try_from(&query.payload).unwrap();
            Ok(CharacterAction::TurnLeft(payload))
        }
        QueryKind::TurnRight => {
            let payload = TryFrom::try_from(&query.payload).unwrap();
            Ok(CharacterAction::TurnRight(payload))
        }
        _ => unimplemented!(),
    }
}
