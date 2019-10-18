use crate::models::character;
use crate::models::item;

pub enum Entity {
    Character(character::CharacterLocal),
    Item, // TODO: item::ItemLocal
}
