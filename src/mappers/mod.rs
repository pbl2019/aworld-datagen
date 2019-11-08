use crate::actions::character_action::*;
use crate::query::*;

mod backward;
mod forward;

pub fn query_to_action(query: &Query) -> Result<CharacterAction, &'static str> {
    match query.kind {
        QueryKind::Forward => {
            CharacterAction::Forward(CharacterForwardPayload::try_from(query.payload)?)
        }
        QueryKind::Backward => {
            CharacterAction::Backward(CharacterBackwardPayload::try_from(query.payload)?)
        }
    }
}
