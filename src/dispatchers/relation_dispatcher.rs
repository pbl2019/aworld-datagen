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
	let relation = Relation{id:1, character_id:1, target_id:2, factor: 0.25};
	let mut rl = RelationLocal::from(relation);
	let mut pl = RelationIncreasePayloac({amout: 32})

	let res = RelationDispatcher::effect_increase(rl,pl)
	assert!(res.is_ok())

	// let charaAlpha = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
 //    let mut character = CharacterLocal::from(mock);
 //    let sleep_state = Sleeping { state: SleepingState::GettingUp, depth: 100 };
 //    character.sleep_state.write(sleep_state);
 //    let res = CharacterDispatcher::action_sleep(&mut character);
 //    assert!(res.is_ok());
 //    assert!(character.sleep_state.read().state == SleepingState::Sleeping);
 //    assert!(character.sleep_state.read().depth == MAX_SLEEP_AMOUNT);
}