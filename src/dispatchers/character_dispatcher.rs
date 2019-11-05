use crate::actions::character_action::*;
use crate::models::character::*;
use std::io::Result;
use std::f32::consts::PI;

struct CharacterDispatcher;
impl CharacterDispatcher {
    fn action_forward(store: &mut CharacterLocal, payload: &CharacterForwardPayload) -> Result<()> {
        let mut x = store.x.read() + store.angle.read().sin() * payload.speed;
        let mut y = store.y.read() + store.angle.read().cos() * payload.speed;
        store.x.write(x);
        store.y.write(y);
        Ok(())
    }
    fn action_backward(store: &mut CharacterLocal, payload: &CharacterBackwardPayload) -> Result<()> {
        let mut x = store.x.read() - store.angle.read().sin() * payload.speed;
        let mut y = store.y.read() - store.angle.read().cos() * payload.speed;
        store.x.write(x);
        store.y.write(y);
        Ok(())
    }
    fn action_turn_left(store: &mut CharacterLocal, payload: &CharacterTurnLeftPayload) -> Result<()> {
        let mut angle = store.angle.read() - payload.angle;
        if angle < 0. {
            angle = (2.*PI - angle).rem_euclid(2.*PI);
        }
        store.angle.write(angle);
        Ok(())
    }
    fn action_turn_right(store: &mut CharacterLocal, payload: &CharacterTurnRightPayload) -> Result<()> {
        let mut angle = store.angle.read() + payload.angle;
        if angle >= 2.*PI {
            angle = angle.rem_euclid(2.*PI);
        }
        store.angle.write(angle);
        Ok(())
    }
    fn effect_pushed(store: &mut CharacterLocal, payload: &CharacterPushedPayload) -> Result<()> {
        let mut x = store.x.read() +  store.angle.read().sin() * payload.speed;
        let mut y = store.y.read() +  store.angle.read().cos() * payload.speed;
        store.x.write(x);
        store.y.write(y);
        Ok(())
    }
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
fn dispatch_action_forward_up() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    character.angle.write(0.);
    let action = CharacterForwardPayload { speed: 1. };
    character.y.write(0.);
    let res = CharacterDispatcher::action_forward(&mut character, &action);
    assert!(res.is_ok());
    assert!(character.y.read() == 1.);
}

#[test]
fn dispatch_action_forward_cross() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    character.angle.write(0.3*PI);
    let action = CharacterForwardPayload { speed: 1. };
    character.y.write(0.);
    let res = CharacterDispatcher::action_forward(&mut character, &action);
    assert!(res.is_ok());
    assert!((character.y.read()*1000.).trunc() == 587.);
}

#[test]
fn dispatch_action_backward_up() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    character.angle.write(0.);
    let action = CharacterBackwardPayload { speed: 1. };
    character.y.write(0.);
    let res = CharacterDispatcher::action_backward(&mut character, &action);
    assert!(res.is_ok());
    assert!(character.y.read() == -1.);
}


#[test]
fn dispatch_action_backward_cross() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    character.angle.write(0.3*PI);
    let action = CharacterBackwardPayload { speed: 1. };
    character.y.write(0.);
    let res = CharacterDispatcher::action_backward(&mut character, &action);
    assert!(res.is_ok());
    assert!((character.y.read()*1000.).trunc() == -587.);
}

#[test]
fn dispatch_action_turn_left() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    character.angle.write(0.3);
    let action = CharacterTurnLeftPayload { angle: 0.1 };
    let res = CharacterDispatcher::action_turn_left(&mut character, &action);
    assert!(res.is_ok());
    assert!((character.angle.read()-0.2).abs() < 0.01);
}

#[test]
fn dispatch_action_turn_right() {
    let mock = Character { id: 001, name: "tset".to_string(), max_hp: 100, max_appetite: 200 };
    let mut character = CharacterLocal::from(mock);
    character.angle.write(0.3);
    let action = CharacterTurnRightPayload { angle: 0.1 };
    let res = CharacterDispatcher::action_turn_right(&mut character, &action);
    assert!(res.is_ok());
    assert!((character.angle.read()-0.4).abs() < 0.01);
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
