use crate::context::Context;
use crate::models::character::CharacterLocal;
use crate::models::item::ItemLocal;
use crate::models::{Object, ObjectId, Entity, EntityId}

impl Context {
    fn fetch_entity(&self, id: EntityId) -> Entity {
        match id {
            EntityId::Character(id) => {
                self.characters.get(id).unwrap().clone()
            },
            EntityId::Item(id) => {
                self.items.get(id).unwrap().clone()
            },
            EntityId::Terrain(id) => {
                self.items.get(id).unwrap().clone()
            },
            EntityId::Relation(id) => {
                self.relations.get(id).unwrap().clone()
            }
        }
    }
    fn fetch_entities(&self, ids: Vec<EntityId>) -> Vec<Entity> {
        ids.iter().map(|id| {
            self.fetch_entity(id)
        }).collect()
    }
    fn fetch_object(&self, id: ObjectId) -> Object {
            match id {
                ObjectId::Character(id) => {
                    self.characters.get(id).unwrap()
                },
                ObjectId::Item(id) => {
                    self.items.get(id).unwrap()
                }
            }
    }
    fn fetch_objects(&self, ids: Vec<ObjectId>) -> Vec<Object> {
        ids.iter().map(|id| {
            self.fetch_object(id)
        }).collect()
    }
}
