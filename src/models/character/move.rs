use crate::models::character::CharacterLocal;
use std::f32;

impl CharacterLocal {
    fn CharacterLocal::move_forward(&mut self) {
        self.x += self.angle.cos();
        self.y += self.angle.sin();
    }
    fn CharacterLocal::move_back(&mut self) {
        self.x -= self.angle.cos();
        self.y -= self.angle.sin();
    }
}