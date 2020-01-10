use crate::models::item::*;
use crate::models::{terrain::*, Entity};
use crate::context::Context;
use crate::counter;
use std::io::Result;
use std::sync::{Arc, RwLock};

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
        let context = Arc::new(RwLock::new(Context::new(terrain_local)));
        {
            let new_item = NewItem::default();
            let item = Item {
                id: 1,
                name: new_item.name,
                item_type: ItemType::Food,
                amount: new_item.amount,
            };
            let item_local = ItemLocal::from(item);
            item_local.x.write(10.0);
            item_local.y.write(10.0);
            context
                .write()
                .unwrap()
                .insert_entity(Entity::Item(Arc::new(item_local)));
        }
        
        return context;
    }

    pub fn generate_meet(context: &mut Context, x: f32, y: f32) -> Result<()> {
        {
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

            context.insert_entity(Entity::Item(Arc::new(item_local)));
        }

        Ok(())
    }
}
