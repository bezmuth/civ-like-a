use amethyst::ecs::prelude::{DenseVecStorage, Component};

// * you might be here because you're getting some error relating to "& cant be mut" or something
// * this means you forgot to use &mut players in the .join iterator!

pub struct Player{
    pub num: i32,
    pub wood: i32,
    pub metal: i32,
    pub science: i32,
}
impl Player {
    pub fn new(num: i32) -> Player {
        Player {
            num,
            wood: 50,
            metal: 25,
            science: 0,
        }
    }

    pub fn sub_wood(&mut self, amount : i32) -> bool {
        let result = self.wood - amount;
        if result < 0{
            return false
        } else {
            self.wood = result;
            return true
        }
    }

    pub fn sub_metal(&mut self, amount : i32) -> bool {
        let result = self.metal - amount;
        if result < 0{
            return false
        } else {
            self.metal = result;
            return true
        }
    }

}
impl Component for Player { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}

pub struct PlayersInfo{
    pub count: i32,
    pub current_player_num : i32,
}
