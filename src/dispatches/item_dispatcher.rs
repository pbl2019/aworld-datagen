struct ItemDispatcher;
impl ItemDispatcher {
	fn effect_spend(store: &mut ItemLocal, payload: &ItemDamagePayload) -> Result<()> {
		store.update_is_used(true);
		Ok(())
	}
	fn effect_break(store: &mut ItemLocal, payload: &ItemDamagePayload) -> Result<()> {
			store.update_amount(store.amount - payload.amount);
		if store.amount - payload.amount <= 0{
			store.update_is_used(true);
		}
		Ok(())
	}
}