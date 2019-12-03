use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref COUNT_MUT: RwLock<u64> = RwLock::new(0);
}

pub fn get_count() -> u64 {
    let current = *COUNT_MUT.read().unwrap();
    *COUNT_MUT.write().unwrap() += 1;
    current
}

#[test]
fn test_counter() {
    assert!(get_count() < get_count());
}
