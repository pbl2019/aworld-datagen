pub mod character_action;
pub mod item_action;
pub mod relation_action;
pub mod system_action;

#[derive(Debug)]
pub enum Action {
    Character(character_action::CharacterAction),
    System(system_action::SystemAction),
}
