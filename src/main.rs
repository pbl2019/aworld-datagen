use aworld_datagen::actions::*;
use aworld_datagen::models;
use aworld_datagen::utils::generate_random_name;

fn main() {
    println!("Hello, world!\n{}", generate_random_name(0));
    println!("{:?}", CharacterEffect::Damage { amount: 10 });
}
