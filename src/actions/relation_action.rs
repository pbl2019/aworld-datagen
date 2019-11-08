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
	pub amount: f64,
}

#[derive(Debug)]
pub struct RelationDecreasePayload {
	pub amount: f64,
}
