use crate::actions::*;
use crate::models::character_action::*;

struct CharacterDispatcher;
impl CharacterDispatcher {
	fn effect_damage(store: &mut CharacterLocal, payload: &CharacterDamagePayload) -> Result<()> {
		let mut hp = store.hp.read() - payload.amount;
		store.hp.write(hp);
		if hp <= 0 {
			store.dead.write(true);
		}
		Ok(())
	}
	fn effect_dead(store: &mut CharacterLocal, payload: &CharacterDamagePayload) -> Result<()> {
		store.dead.write(true);
		Ok(())
	}
}

#[test]
fn testCharacter(){
	let cd = CharacterDispatcher();
	let cl = CharacterLocal(NewCharacter::default())
	assert!(cd.effect_damage(cl,CharacterDamagePayload({amount: 10})) != 0)
	assert!(cd.effect_damage(cl,CharacterDamagePayload({amount: 10})) == Ok(()))
	assert!(cd.effect_damage(cl,CharacterDamagePayload({amount: 10})) != -1)
	assert!(cd.effect_damage(cl,CharacterDamagePayload({amount: 10})) != "a")
	assert!(cd.effect_damage(cl,CharacterDamagePayload({amount: 10})) != "null")
}

#[test]
fn test_character() {
    let new_character = NewCharacter::default();
    assert!(new_character.name.len() > 0);
    assert!(new_character.max_hp > 0);
    assert!(new_character.max_appetite > 0);
}
