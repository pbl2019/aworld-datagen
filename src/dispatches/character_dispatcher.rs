struct CharacterDispatcher;
impl CharacterDispatcher {
    fn effect_damage(store: &mut CharacterLocal, payload: &CharacterDamagePayload) -> Result<()> {
        store.update_hp(store.hp - payload.amount);
        if store.hp - payload.amount <= 0{
            store.update_dead(true);
        }
        Ok(())
    }
    fn effect_dead(store: &mut CharacterLocal, payload: &CharacterDamagePayload) -> Result<()> {
    	store.update_dead(true);
        Ok(())
    
    }
}
