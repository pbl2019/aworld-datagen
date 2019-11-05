#[derive(Debug)]
pub enum RelationAction {
	
}

#[derive(Debug)]
pub enum RelationEffect {
	Increase(RelationIncreasePayload),
	Decrease(RelationDecreasePayload),
}

#[derive(Debug)]
pub struct RelationIncreasePayload {
	amount: f32,
}

#[derive(Debug)]
pub struct RelationDecreasePayload {
	amount: f32,
}