#[derive(Debug)]
pub enum SystemAction {
    Login(SystemLoginPayload),
}

#[derive(Debug)]
pub struct SystemLoginPayload {
    pub character_id: Option<u64>,
    pub password: Option<String>,
}
