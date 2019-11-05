use crate::actions::character_action::*;
use crate::models::character::*;
use std::io::Result;

struct CharacterDispatcher;
impl CharacterDispatcher {

    fn action_sleep(store: &mut CharacterLocal) -> Result<()> {
        let sleep_state = Sleeping {
            state: SleepingState::Sleeping,
            depth: MAX_SLEEP_AMOUNT
        };
        store.sleep_state.write(sleep_state);
        Ok(())
    }
    fn action_rest(store: &mut CharacterLocal) -> Result<()> {
        let sleep_state = Sleeping {
            state: SleepingState::Sleeping,
            depth: MIN_SLEEP_AMOUNT
        };
        store.sleep_state.write(sleep_state);
        Ok(())
    }
    fn action_idle(store: &mut CharacterLocal) -> Result<()> {
        let sleep_state = Sleeping {
            state: SleepingState::Idle,
            depth: MIN_SLEEP_AMOUNT
        };
        store.sleep_state.write(sleep_state);
        Ok(())
    }
    fn action_get_up(store: &mut CharacterLocal) -> Result<()> {
        let sleep_state = Sleeping {
            state: SleepingState::GettingUp,
            depth: MIN_SLEEP_AMOUNT
        };
        store.sleep_state.write(sleep_state);
        Ok(())
    }

    fn effect_damage(store: &mut CharacterLocal, payload: &CharacterDamagedPayload) -> Result<()> {
        let mut hp = store.hp.read() - payload.amount;
        store.hp.write(hp);
        if hp <= 0 {
            store.is_dead.write(true);
        }
        Ok(())
    }
    fn effect_dead(store: &mut CharacterLocal, payload: &CharacterDamagedPayload) -> Result<()> {
        store.is_dead.write(true);
        Ok(())
    }
    fn effect_recovery(store: &mut CharacterLocal, payload: &CharacterRecoveryPayload) -> Result<()> {
        let hp = store.model.max_hp.max(store.hp.read() + payload.depth);
        store.hp.write(hp);
        Ok(())
    }
    fn effect_wakeup(store: &mut CharacterLocal) -> Result<()> {
        let sleep_state = Sleeping {
            state: SleepingState::GettingUp,
            depth: store.sleep_state.read().depth / 2
        };
        store.sleep_state.write(sleep_state);
        Ok(())
    }
    fn effect_disturb(store: &mut CharacterLocal) -> Result<()> {
        let sleep_state = Sleeping {
            state: SleepingState::GettingUp,
            depth: store.sleep_state.read().depth
        };
        store.sleep_state.write(sleep_state);
        Ok(())
    }
}

#[test]
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
#[test]
fn dispatch_action_rest() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    let sleep_state = Sleeping { state: SleepingState::GettingUp, depth: 100 };
    character.sleep_state.write(sleep_state);
    let res = CharacterDispatcher::action_rest(&mut character);
    assert!(res.is_ok());
    assert!(character.sleep_state.read().state == SleepingState::Sleeping);
    assert!(character.sleep_state.read().depth == MIN_SLEEP_AMOUNT);
}
#[test]
fn dispatch_action_idle() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    let sleep_state = Sleeping { state: SleepingState::Sleeping, depth: 100 };
    character.sleep_state.write(sleep_state);
    let res = CharacterDispatcher::action_idle(&mut character);
    assert!(res.is_ok());
    assert!(character.sleep_state.read().state == SleepingState::Idle);
    assert!(character.sleep_state.read().depth == MIN_SLEEP_AMOUNT);
}
#[test]
fn dispatch_action_get_up() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    let sleep_state = Sleeping { state: SleepingState::Sleeping, depth: 100 };
    character.sleep_state.write(sleep_state);
    let res = CharacterDispatcher::action_get_up(&mut character);
    assert!(res.is_ok());
    assert!(character.sleep_state.read().state == SleepingState::GettingUp);
    assert!(character.sleep_state.read().depth == MIN_SLEEP_AMOUNT);
}
#[test]
fn dispatch_effect_recovery() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    let action = CharacterRecoveryPayload { depth: 20 };
    character.hp.write(80);
    let res = CharacterDispatcher::effect_recovery(&mut character, &action);
    assert!(res.is_ok());
    assert!(character.hp.read() == 100);
}
#[test]
fn dispatch_effect_wakeup() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    let sleep_state = Sleeping { state: SleepingState::Sleeping, depth: 100 };
    character.sleep_state.write(sleep_state);
    let res = CharacterDispatcher::effect_wakeup(&mut character);
    assert!(res.is_ok());
    assert!(character.sleep_state.read().state == SleepingState::GettingUp);
    assert!(character.sleep_state.read().depth == 50);
}
#[test]
fn dispatch_effect_disturb() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    let sleep_state = Sleeping { state: SleepingState::Sleeping, depth: 3000 };
    character.sleep_state.write(sleep_state);
    let res = CharacterDispatcher::effect_disturb(&mut character);
    assert!(res.is_ok());
    assert!(character.sleep_state.read().state == SleepingState::GettingUp);
    assert!(character.sleep_state.read().depth == 3000);
}
