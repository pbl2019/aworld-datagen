use crate::actions::character_action::*;
use serde_json::value::Value;

static EXPECTION: &'static str = r#"{"angle": float}"#;

impl std::convert::TryFrom<&Value> for CharacterTurnLeftPayload {
    type Error = &'static str;
    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        match payload {
            Value::Object(map) => {
                if let Some(angle) = map.get("angle") {
                    Ok(CharacterTurnLeftPayload {
                        angle: angle.as_f64().unwrap() as f32,
                    })
                } else {
                    Err(EXPECTION)
                }
            }
            _ => Err(EXPECTION),
        }
    }
}
