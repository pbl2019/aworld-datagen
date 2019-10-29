struct RelationDispatcher;
impl RelationDispatcher {
	fn effect_befriend(store: &mut CharacterLocal, payload: &CharacterDamagePoyload) -> Result<()> {
		store.update_factor(store.factor + payload.amount);
		Ok(())
	}
	fn effect_worsening(store: &mut CharacterLocal, payload: &CharacterDamagePoyload) -> Result<()> {
		store.update_factor(store.factor  payload.amount);
		Ok(())
	}
}