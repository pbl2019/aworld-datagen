#[derive(Debug)]
pub enum ItemAction {
}

#[derive(Debug)]
pub enum ItemEffect {
    Spend(ItemSpendPayload),
    Break,
}

#[derive(Debug)]
pub struct ItemSpendPayload {
    amount: f32,
}

