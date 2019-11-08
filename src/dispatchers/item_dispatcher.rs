use crate::actions::item_action::*;
use crate::models::item::*;
use std::io::Result;

struct ItemDispatcher;
impl ItemDispatcher {
	fn effect_spend(store: &mut ItemLocal) -> Result<()> {
		store.is_used.write(true);
		Ok(())
	}
	fn effect_break(store: &mut ItemLocal, payload: &ItemBreakPayload) -> Result<()> {
		let durability = store.durability.read() - payload.durability;
		store.durability.write(durability);
		if durability <= 0 {
			store.is_used.write(true);
		}
		Ok(())
	}
}

#[test]
fn test_effect_spend() {
	let mock = Item {id: 0, name: "Wao!!な薬".to_string(), item_type: ItemType::Food, amount: 10};
	let mut item = ItemLocal::from(mock);
	let res = ItemDispatcher::effect_spend(&mut item);
	assert!(res.is_ok());
	assert!(item.is_used.read() == true);
}

#[test]
fn test_effect_break() {
	let mock = Item {id: 1, name: "Wao!?な薬".to_string(), item_type: ItemType::Food, amount: 15};
	let mut item = ItemLocal::from(mock);
	let payload = ItemBreakPayload{durability: item.durability.read() - 10};
	let res = ItemDispatcher::effect_break(&mut item, &payload);
	assert!(res.is_ok());
	assert!(item.durability.read() == 10);
	assert!(item.is_used.read() != true);
	let res = ItemDispatcher::effect_break(&mut item, &payload);
	assert!(item.is_used.read() == true);
}
