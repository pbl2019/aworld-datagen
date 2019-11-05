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
