use crate::models::character::CharacterLocal;
use crate::models::item::ItemLocal;
use std::collections::HashMap;
use std::sync::Arc;

pub type Repository<T> = HashMap<i64, Arc<T>>;

pub struct Context {
    characters: Repository<CharacterLocal>,
    items: Repository<ItemLocal>,
}
