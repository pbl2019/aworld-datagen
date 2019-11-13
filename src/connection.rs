#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Connection {
    pub addr: String,
    pub salt: i64,
}
