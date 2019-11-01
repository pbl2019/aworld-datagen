struct CharacterDispatcher;

#[derive(Debug)]
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
}