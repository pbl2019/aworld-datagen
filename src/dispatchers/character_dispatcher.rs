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
