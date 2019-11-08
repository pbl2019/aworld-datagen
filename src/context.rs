use crate::connection::Connection;
use crate::models::character::CharacterLocal;
use crate::models::item::ItemLocal;
use std::collections::HashMap;
use std::sync::Arc;

pub type Repository<T> = HashMap<i64, Arc<T>>;

pub struct Context {
    login_infos: HashMap<String, i64>,
    characters: Repository<CharacterLocal>,
    items: Repository<ItemLocal>,
}

impl Context {
    fn new() -> Self {
        Self {
            login_infos: HashMap::new(),
            characters: HashMap::new(),
            items: HashMap::new(),
        }
    }
    fn get_character_from_connection(&self, conn: &Connection) -> Option<Arc<CharacterLocal>> {
        self.login_infos
            .get(&conn.addr)
            .and_then(|id| self.characters.get(&id))
            .and_then(|p| Some(p.clone()))
    }
}
