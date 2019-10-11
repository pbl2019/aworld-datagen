use crate::schema::items;
use crate::utils::define_enum;

define_enum! {
    #[derive(Debug, Clone, Copy)]
    pub enum ItemType {
        Unknown = 0,
        Food = 1,
        Weapon = 2,
    }
}

#[derive(Queryable)]
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

impl std::default::Default for NewItem {
    fn default() -> Self {
        Self {
            name: "bar".to_owned(),
            item_type: ItemType::Unknown,
            amount: 0,
        }
    }
}

#[test]
fn create_item() {
    let new_item = NewItem::default();
    assert!(new_item.name.len() > 0);
    // item_type must be lower than number of all kinds of item type
    assert!(new_item.item_type < 12);
    assert!(new_item.amount >= 0);
}
