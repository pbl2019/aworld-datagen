use crate::connection::Connection;
use crate::models::character::CharacterLocal;
use crate::models::item::ItemLocal;
use crate::models::terrain::TerrainLocal;
use std::collections::HashMap;
use std::sync::Arc;

pub type Repository<T> = HashMap<i64, Arc<T>>;

pub struct Context {
    pub login_infos: HashMap<String, i64>,
    pub characters: Repository<CharacterLocal>,
    pub items: Repository<ItemLocal>,
    pub terrain: Arc<TerrainLocal>,
}

impl Context {
    pub fn new(terrain: TerrainLocal) -> Self {
        Self {
            login_infos: HashMap::new(),
            characters: HashMap::new(),
            items: HashMap::new(),
            terrain: Arc::new(terrain),
        }
    }
    pub fn get_character_from_connection(&self, conn: &Connection) -> Option<Arc<CharacterLocal>> {
        self.login_infos
            .get(&conn.addr)
            .and_then(|id| self.characters.get(&id))
            .and_then(|p| Some(p.clone()))
    }
}
