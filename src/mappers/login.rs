use crate::actions::system_action::*;
use serde_json::value::Value;

static EXPECTION: &'static str = r#"{"character_id": integer | null, "password": string | null}"#;

impl std::convert::TryFrom<&Value> for SystemLoginPayload {
    type Error = String;
    fn try_from(payload: &Value) -> Result<Self, Self::Error> {
        match payload {
            Value::Object(map) => {
                let mut character_id = None;
                let mut password = None;
                if let Some(cid) = map.get("character_id") {
                    if cid.is_i64() {
                        character_id = Some(cid.as_u64().unwrap());
                    }
                }
                if let Some(pw) = map.get("password") {
                    if pw.is_string() {
                        password = Some(pw.as_str().unwrap().to_owned());
                    }
                }
                Ok(SystemLoginPayload {
                    character_id,
                    password,
                })
            }
            _ => Err(EXPECTION.to_owned()),
        }
    }
}
