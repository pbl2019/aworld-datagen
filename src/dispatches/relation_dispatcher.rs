struct RelationDispatcher;
impl RelationDispatcher {
	fn effect_increase(store: &mut RelationLocal, payload: &RelationPayload) -> Result<()> {
		store.update_factor(store.factor + payload.amount);
		Ok(())
	}
	fn effect_decrease(store: &mut RelationLocal, payload: &RelationPayload) -> Result<()> {
		store.update_factor(store.factor - payload.amount);
		Ok(())
	}
}

#[test]
fn test_dispatch(){
	dispatcher = &RelationDispatcher;
}