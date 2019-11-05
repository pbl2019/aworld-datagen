use serde::de::{self, Visitor};
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub enum QueryKind {
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
            "attack" => QueryKind::Attack,
            _ => QueryKind::Unknown(s),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Query {
    character_id: String,
    kind: QueryKind,
    payload: Value,
}
