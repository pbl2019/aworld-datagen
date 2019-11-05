use crate::actions::item_action::*;
use crate::models::item::ItemLocal;

struct ItemDispatcher;
impl ItemDispatcher {
    fn effect_spend(store: &mut ItemLocal, payload: &ItemDamagePayload) -> Result<()> {
        store.is_used.write(true);
        Ok(())
    }
    fn effect_break(store: &mut ItemLocal, payload: &ItemDamagePayload) -> Result<()> {
        let durability = store.durability - payload.amount;
        store.durability.write(durability);
        if durability <= 0 {
            store.is_used.write(true);
        }
        Ok(())
    }
}

#[test]
fn test(){
    let mock = Item {name: "Wao!!な薬", item_type: 1, amount: 10}
}
fn dispatch_action_sleep() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    let sleep_state = Sleeping { state: SleepingState::GettingUp, depth: 100 };
    character.sleep_state.write(sleep_state);
    let res = CharacterDispatcher::action_sleep(&mut character);
    assert!(res.is_ok());
    assert!(character.sleep_state.read().state == SleepingState::Sleeping);
    assert!(character.sleep_state.read().depth == MAX_SLEEP_AMOUNT);
}
