use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Copy, Clone, Default)]
pub struct Stat{
    pub health: f32,
    pub attack: f32,
    pub resistance: f32, // chance to not take damage
    pub regen: f32,
    pub vampire: f32,
    pub multi_hit_chance: f32, // chance to multi hit
    pub multi_hit_amount: f32, // how many times to multi hit
    pub crit_chance: f32, //chance to do a random crit
}


// impl Stat{
//    fn attack(&mut self, other : Stat){
//        let damage_done = other.damage(self);
//    }
//    fn damage(&mut s other : Stat){
//    }
//}
impl Component for Stat {
    type Storage = DenseVecStorage<Self>;
}
