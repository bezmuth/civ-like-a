use crate::game::{CurrentPlayer, Player, Building, BuildingType};

use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadExpect},
};





#[derive(SystemDesc)]
pub struct ResourceDispSystem;

impl<'s> System<'s> for ResourceDispSystem {
    type SystemData = (
        ReadExpect<'s, CurrentPlayer>,
        WriteStorage<'s, Player>,
    );


    fn run(&mut self, (currentplayer, buildings, mut players): Self::SystemData) {
        for player in (&mut players).join(){
            if player.num == currentplayer.playernum {

            }
        }
    }
}

