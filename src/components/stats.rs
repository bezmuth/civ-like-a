use amethyst::ecs::{Component, DenseVecStorage};


#[derive(Copy, Clone, Default)]
pub struct Stat{
    pub speed: f32, // how far the unit can move in a single turn
    pub health: f32,
    pub attack: f32,
    pub resistance: f32, // chance to not take damage
    pub multi_hit_chance: f32, // chance to multi hit
    pub multi_hit_amount: f32, // how many times to multi hit
    pub crit_chance: f32, //chance to do a random crit
    pub exhausted: bool,
}


impl Component for Stat {
    type Storage = DenseVecStorage<Self>;
}
