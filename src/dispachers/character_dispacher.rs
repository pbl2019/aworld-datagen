src/dispatchers/character_dispatcher.rs

struct CharacterDispatcher;
impl CharacterDispatcher {
    fn effect_damage(store: &mut CharacterLocal, payload: &CharacterDamagePoyload) -> Result<()> {
        store.update_hp(store.hp - payload.amount);
        Ok(())
    }
    fn effect_dead(store: &mut CharacterLocal, payload: &CharacterDamagePoyload) -> Result<()> {
    	store.update_dead(true);
        Ok(())
    
    }
}