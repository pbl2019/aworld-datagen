#![allow(dead_code)]
use crate::connection::Connection;
use crate::log;
use crate::models::character::CharacterLocal;
use crate::models::item::ItemLocal;
use crate::models::relation::RelationLocal;
use crate::models::terrain::TerrainLocal;
use crate::models::Entity;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub type Repository<T> = HashMap<i64, Arc<T>>;

pub struct Context {
    entities: HashMap<i64, Entity>,
    mutated_entity_ids: HashSet<i64>,
    pub connection_to_character_id: HashMap<Connection, i64>,
    pub characters: Repository<CharacterLocal>,
    pub items: Repository<ItemLocal>,
    pub relations: Repository<RelationLocal>,
    pub terrain: Arc<TerrainLocal>,
}

impl Context {
    pub fn new(terrain: TerrainLocal) -> Self {
        let terrain = Arc::new(terrain);
        let mut entities = HashMap::new();
        entities.insert(terrain.model.id, terrain.clone());
        Self {
            entities: HashMap::new(),
            mutated_entity_ids: HashSet::new(),
            connection_to_character_id: HashMap::new(),
            characters: HashMap::new(),
            items: HashMap::new(),
            relations: HashMap::new(),
            terrain,
        }
    }
    pub fn get_character_from_connection(
        &self,
        conn: &Connection,
    ) -> Result<Arc<CharacterLocal>, String> {
        self.connection_to_character_id
            .get(&conn)
            .and_then(|id| self.characters.get(&id))
            .and_then(|p| Some(p.clone()))
            .ok_or(format!(
                "{:?} has not been associated to any character",
                conn
            ))
    }
    pub fn insert_entity(&mut self, entity: Entity) {
        let entity_id;
        match entity.clone() {
            Entity::Character(local) => {
                self.characters.insert(local.model.id, local.clone());
                entity_id = local.entity_id;
            }
            Entity::Item(local) => {
                self.items.insert(local.model.id, local.clone());
                entity_id = local.entity_id;
            }
            Entity::Terrain(local) => {
                self.terrain = local.clone();
                entity_id = local.entity_id;
            }
            Entity::Relation(local) => {
                self.relations.insert(local.model.id, local.clone());
                entity_id = local.entity_id;
            }
        }
        // TODO: Remove here if secure
        if let Some(_) = self.entities.get(&entity_id) {
            log!("BUG", "Same entity id has been found");
            panic!("same entity id has been found");
        }
        self.entities.insert(entity_id, entity.clone());
    }
    pub fn mark_mutations(&mut self, mutated_entity_ids: Vec<i64>) {
        for id in mutated_entity_ids.into_iter() {
            self.mutated_entity_ids.insert(id);
        }
    }
    pub fn get_mutated_entities(&mut self) -> Vec<Entity> {
        let mut res = Vec::new();
        for id in self.mutated_entity_ids.drain() {
            let entity = self.entities.get(&id).unwrap();
            res.push(entity.clone());
        }
        res
    }
}
