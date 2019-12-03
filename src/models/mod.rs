use std::sync::Arc;

pub mod character;
mod field;
pub mod item;
// pub mod ray;
pub mod relation;
pub mod terrain;

pub type entity_id = u64;

#[derive(Copy, Clone, Debug)]
pub enum ObjectId {
    Character(u64),
    Item(u64),
}

#[derive(Copy, Clone, Debug)]
pub enum EntityId {
    Character(u64),
    Item(u64),
    Terrain(u64),
    Relation(u64),
}

#[derive(Clone, Debug)]
pub enum Object {
    Character(Arc<character::CharacterLocal>),
    Item(Arc<item::ItemLocal>),
}

#[derive(Clone, Debug)]
pub enum Entity {
    Character(Arc<character::CharacterLocal>),
    Item(Arc<item::ItemLocal>),
    Terrain(Arc<terrain::TerrainLocal>),
    Relation(Arc<relation::RelationLocal>),
}
