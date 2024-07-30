use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current_hp: i32,
    pub max_hp: i32,
}

impl Health {
    pub fn new(max_hp: i32) -> Self {
        Self {
            current_hp: max_hp,
            max_hp,
        }
    }

    pub fn damage(&mut self, amount: i32) {
        self.current_hp -= amount;
    }

    pub fn is_dead(&self) -> bool {
        self.current_hp <= 0
    }
}
