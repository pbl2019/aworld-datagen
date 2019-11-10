use crate::actions::relation_action::*;
use crate::models::relation::*;
use std::io::Result;

pub struct RelationDispatcher;
impl RelationDispatcher {
    pub fn effect_increase(store: &RelationLocal, payload: &RelationIncreasePayload) -> Result<()> {
        store.factor.write(store.factor.read() + payload.amount);
        Ok(())
    }
    pub fn effect_decrease(store: &RelationLocal, payload: &RelationDecreasePayload) -> Result<()> {
        store.factor.write(store.factor.read() - payload.amount);
        Ok(())
    }
}

#[test]
fn test_effect_increase() {
    let mock = Relation {
        id: 0,
        character_id: 0,
        target_id: 1,
        factor: 10.,
    };
    let mut relation = RelationLocal::from(mock);
    let payload = RelationIncreasePayload { amount: 5. };
    let res = RelationDispatcher::effect_increase(&mut relation, &payload);
    assert!(res.is_ok());
    assert!(relation.factor.read() == 15.);
}

#[test]
fn test_effect_decrease() {
    let mock = Relation {
        id: 0,
        character_id: 0,
        target_id: 1,
        factor: 10.,
    };
    let mut relation = RelationLocal::from(mock);
    let payload = RelationDecreasePayload { amount: 5. };
    let res = RelationDispatcher::effect_decrease(&mut relation, &payload);
    assert!(res.is_ok());
    assert!(relation.factor.read() == 5.);
}
