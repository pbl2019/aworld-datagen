#[derive(Debug)]
pub enum ItemAction {
}

#[derive(Debug)]
pub enum ItemEffect {
    Spend,
    Break(ItemBreakPayload),
}

#[derive(Debug)]
pub struct ItemBreakPayload {
    durability: f32,
}

