#[derive(Debug)]
pub enum SystemAction {
    Login(SystemLoginPayload),
}

#[derive(Debug)]
pub struct SystemLoginPayload {
    pub character_id: Option<i64>,
    pub password: Option<String>,
}
