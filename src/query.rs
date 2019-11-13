use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub enum QueryKind {
    Login,
    Attack,
    Forward,
    Backward,
    TurnLeft,
    TurnRight,
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for QueryKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "login" => QueryKind::Login,
            "attack" => QueryKind::Attack,
            "forward" => QueryKind::Forward,
            "backward" => QueryKind::Backward,
            "turn_left" => QueryKind::TurnLeft,
            "turn_right" => QueryKind::TurnRight,
            _ => QueryKind::Unknown(s),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub salt: i64,
    pub addr: String,
    pub kind: QueryKind,
    pub payload: Value,
}
