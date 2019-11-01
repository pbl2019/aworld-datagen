use crate::actions::*;
use crate::models::character_action::*;

struct CharacterDispatcher;
impl CharacterDispatcher {
  fn forward(store: &mut CharacterLocal, payload: &CharacterForwardPayload) -> Result<()> {
        store.update_pos(store.x + store.angle.sin() * payload.speed, store.y + store.angle.cos() * payload.speed);
        Ok(())
    }
    fn backward(store: &mut CharacterLocal, payload: &CharacterBackwardPayload) -> Result<()> {
        store.update_pos(store.x - store.angle.sin() * payload.speed, store.y - store.angle.cos() * payload.speed);
        Ok(())
    }
    fn turn_left(store: &mut CharacterLocal, payload: &CharacterTurnLeftPayload) -> Result<()> {
        store.update_angle(store.angle - payload.angle);
        Ok(())
    }
    fn turn_right(store: &mut CharacterLocal, payload: &CharacterTurnRightPayload) -> Result<()> {
        store.update_angle(store.angle + payload.angle);
        Ok(())
    }
    fn effect_pushed(store: &mut CharacterLocal, payload: &CharacterPushedPayload) -> Result<()> {
        store.update_pos(store.x + payload.angle.sin() * payload.speed, store.y + payload.angle.cos() * payload.speed);
        Ok(())
    }
    fn effect_damage(store: &mut CharacterLocal, payload: &CharacterDamagePayload) -> Result<()> {
        let mut hp = store.hp.read() - payload.amount;
        store.hp.write(hp);
        if hp <= 0 {
            store.dead.write(true);
        }
        Ok(())
    }
    fn effect_dead(store: &mut CharacterLocal, payload: &CharacterDamagePayload) -> Result<()> {
        store.dead.write(true);
        Ok(())
    }
}
