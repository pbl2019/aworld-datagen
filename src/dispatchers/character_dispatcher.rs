struct CharacterDispatcher;

impl CharacterDispatcher {
    fn action_sleep(store: &mut CharacterLocal) -> Result<()> {
        Ok(())
    }

    fn action_rest(store: &mut CharacterLocal) -> Result<()> {
        Ok(())
    }

    fn action_idle(store: &mut CharacterLocal) -> Result<()> {
        Ok(())
    }

    fn action_get_up(store: &mut CharacterLocal) -> Result<()> {
        Ok(())
    }

    fn effect_damage(store: &mut CharacterLocal, payload: &CharacterDamagePoyload) -> Result<()> {
        store.update_hp(store.hp - payload.amount);
        Ok(())
    }

    fn effect_recovery(store: &mut CharacterLocal, payload: &CharacterRecoveryPayload) -> Result<()> {
        let mut hp = store.hp + payload.amount;
        hp = if store.model.max_hp > hp { store.model.max_hp } 
        store.update_hp(hp);
        Ok(())
    }

    fn effect_wakeup(store: &mut CharacterLocal) -> Result<()> {
        Ok(())
    }

    fn effect_disturb(store: &mut CharacterLocal) -> Result<()> {
        Ok(())
    }
}
