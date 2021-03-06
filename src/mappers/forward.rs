use crate::actions::character_action::CharacterForwardPayload;
use serde_json::value::Value;

static EXPECTION: &'static str = r#"{"speed": float}"#;

impl std::convert::TryFrom<&Value> for CharacterForwardPayload {
    type Error = String;
    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        match payload {
            Value::Object(map) => {
                if let Some(speed) = map.get("speed") {
                    Ok(CharacterForwardPayload {
                        speed: speed.as_f64().unwrap() as f32,
                    })
                } else {
                    Err(EXPECTION.to_owned())
                }
            }
            _ => Err(EXPECTION.to_owned()),
        }
    }
}
