use std::sync::Arc;

pub mod character;
mod field;
pub mod item;
// pub mod ray;
pub mod relation;
pub mod terrain;

#[derive(Copy, Clone, Debug)]
pub enum ObjectId {
    Character(i64),
    Item(i64),
}

#[derive(Copy, Clone, Debug)]
pub enum EntityId {
    Character(i64),
    Item(i64),
    Terrain(i64),
    Relation(i64),
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
