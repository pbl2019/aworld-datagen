use crate::actions::character_action::*;
use serde_json::value::Value;

static EXPECTION: &'static str = r#"{"item_index": integer}"#;

impl std::convert::TryFrom<&Value> for CharacterUsePayload {
    type Error = String;
    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        match payload {
            Value::Object(map) => {
                if let Some(item_index) = map.get("item_index") {
                    item_index
                        .as_i64()
                        .and_then(|item_index| Some(CharacterUsePayload { item_index }))
                        .ok_or(EXPECTION.to_owned())
                } else {
                    Err(EXPECTION.to_owned())
                }
            }
            _ => Err(EXPECTION.to_owned()),
        }
    }
}
