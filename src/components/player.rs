use amethyst::ecs::prelude::{DenseVecStorage, Component};

// * you might be here because you're getting some error relating to "& cant be mut" or something
// * this means you forgot to use &mut players in the .join iterator!

pub struct Player{
    pub num: i8,
    pub wood: i32,
    pub metal: i32,
    pub faith: i32,
}
impl Player {
    pub fn new(num: i8) -> Player {
        Player {
            num,
            wood: 0,
            metal: 0,
            faith: 0,
        }
    }
}
impl Component for Player { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}