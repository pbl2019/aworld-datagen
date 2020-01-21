use crate::models::item::*;
use crate::models::{terrain::*, Entity};
use crate::context::Context;
use crate::counter;
use std::io::Result;
use std::sync::{Arc, RwLock};
use rand::Rng;

#[derive(Debug)]
pub struct Environment {}

impl Environment {
    pub fn new() -> Arc<RwLock<Context>> {
        let new_terrain = NewTerrain::with_size(50, 50);
        let terrain = Terrain {
            id: 0,
            content: new_terrain.content,
            width: new_terrain.width,
            height: new_terrain.height,
        };
        let terrain_local = TerrainLocal::from(terrain);

        let mut rng = rand::thread_rng();
        let max_weapon: i64 = rng.gen_range(0., 20.) as i64;
        let mut weapons_position: Vec<[f32; 2]> = Vec::new();
        for _ in 0..max_weapon {
            let (x, y) = terrain_local.randpos();
            let position = [x, y];
            weapons_position.push(position);
        }

        let context = Arc::new(RwLock::new(Context::new(terrain_local)));
        {
            Self::generate_meet(&mut context.write().unwrap(), 10.0, 10.0).unwrap();
            for [x, y] in weapons_position.into_iter() {
                Self::generate_weapon(&mut context.write().unwrap(), x, y).unwrap();
            }
        }
        
        return context;
    }

    pub fn generate_weapon(context: &mut Context, x: f32, y: f32) -> Result<u64> {
        let new_item = NewItem::default();
        let item = Item {
            id: counter::get_count() as i64,
            name: new_item.name,
            item_type: ItemType::Weapon,
            amount: new_item.amount,
        };
        let item_local = ItemLocal::from(item);
        item_local.x.write(x);
        item_local.y.write(y);
        let entity_id = item_local.entity_id;

        context.insert_entity(Entity::Item(Arc::new(item_local)));
        Ok(entity_id)
    }

    pub fn generate_meet(context: &mut Context, x: f32, y: f32) -> Result<u64> {
        let new_item = NewItem::default();
        let item = Item {
            id: counter::get_count() as i64,
            name: new_item.name,
            item_type: ItemType::Food,
            amount: new_item.amount,
        };
        let item_local = ItemLocal::from(item);
        item_local.x.write(x);
        item_local.y.write(y);
        let entity_id = item_local.entity_id;

        context.insert_entity(Entity::Item(Arc::new(item_local)));
        Ok(entity_id)
    }
}
