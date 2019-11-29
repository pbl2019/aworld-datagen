use crate::context::Context;
use crate::models::*;

impl Context {
    pub fn fetch_entity(&self, id: EntityId) -> Entity {
        match id {
            EntityId::Character(id) => Entity::Character(self.characters.get(&id).unwrap().clone()),
            EntityId::Item(id) => Entity::Item(self.items.get(&id).unwrap().clone()),
            EntityId::Terrain(_) => Entity::Terrain(self.terrain.clone()),
            EntityId::Relation(id) => Entity::Relation(self.relations.get(&id).unwrap().clone()),
        }
    }
    pub fn fetch_entities(&self, ids: Vec<EntityId>) -> Vec<Entity> {
        ids.iter().map(|id| self.fetch_entity(*id)).collect()
    }
    pub fn fetch_object(&self, id: ObjectId) -> Object {
        match id {
            ObjectId::Character(id) => Object::Character(self.characters.get(&id).unwrap().clone()),
            ObjectId::Item(id) => Object::Item(self.items.get(&id).unwrap().clone()),
        }
    }
    pub fn fetch_objects(&self, ids: Vec<ObjectId>) -> Vec<Object> {
        println!("{:?}", ids);
        println!("{:?}", self.characters);
        ids.iter().map(|id| self.fetch_object(*id)).collect()
    }
}
