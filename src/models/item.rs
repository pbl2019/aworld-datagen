#![allow(unused_imports)]
use crate::models::field::Field;
use crate::schema::items;
use crate::utils::generate_random_name;
use crate::{define_enum, init_field};
use chrono::Utc;
use num_traits::{FromPrimitive, ToPrimitive};
use rand::Rng;

define_enum! {
    #[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy)]
    pub enum ItemType {
        Unknown = 0,
        Food = 1,
        Weapon = 2,
    }
}

#[derive(Queryable, Clone, Debug)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub item_type: ItemType,
    pub amount: i64,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
    pub item_type: ItemType,
    pub amount: i64,
}

#[derive(Debug)]
pub struct ItemLocal {
    pub entity_id: i64,
    pub model: Item,

    pub is_used: Field<bool>,
    pub max_durability: Field<i64>, // NOTE: アイテム毎で耐久度が異なる
    pub durability: Field<i64>,
}

impl std::default::Default for NewItem {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            name: generate_random_name(0),
            item_type: ItemType::from_u64(rng.gen_range(0, 2 + 1)).unwrap(),
            amount: rng.gen_range(0, 1000),
        }
    }
}

impl std::convert::From<Item> for ItemLocal {
    fn from(model: Item) -> Self {
        let mut rng = rand::thread_rng();
        let max_durability = rng.gen_range(10, 1000);
        Self {
            entity_id: Utc::now().timestamp(),
            model: model.clone(),
            is_used: init_field!(false),
            max_durability: init_field!(max_durability),
            durability: init_field!(max_durability),
        }
    }
}

#[test]
fn create_item() {
    let new_item = NewItem::default();
    assert!(new_item.name.len() > 0);
    // item_type must be lower than number of all kinds of item type
    assert!(new_item.item_type.to_u32().unwrap() <= 2);
    assert!(new_item.amount >= 0);
}
