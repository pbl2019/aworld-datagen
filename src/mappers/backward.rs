use crate::actions::character_action::CharacterBackwardPayload;
use serde_json::value::Value;

static EXPECTION: &'static str = r#"expect {"speed": float}"#;

impl std::convert::TryFrom<&Value> for CharacterBackwardPayload {
    type Error = &'static str;
    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        match payload {
            Value::Object(map) => {
                if let Some(speed) = map.get("speed") {
                    Ok(CharacterBackwardPayload {
                        speed: speed.as_f64().unwrap() as f32,
                    })
                } else {
                    Err(EXPECTION)
                }
            }
            _ => Err(EXPECTION),
        }
    }
}
