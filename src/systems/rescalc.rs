use crate::game::{CurrentPlayer, Player, Building, BuildingType};

use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadExpect},
};

#[derive(SystemDesc)]
pub struct ResourceCalcSystem;

impl<'s> System<'s> for ResourceCalcSystem {
    type SystemData = (
        ReadExpect<'s, CurrentPlayer>,
        WriteStorage<'s, Building>,
        WriteStorage<'s, Player>,
    );


    fn run(&mut self, (currentplayer, buildings, mut players): Self::SystemData) {
        // TODO: integrate with turn system 
        for building in (buildings).join(){
            if building.playernum == currentplayer.playernum{
                for player in (&mut players).join(){
                    if player.num == currentplayer.playernum {
                        match building.buildingtype { // TODO: Ensure these are balanced
                            BuildingType::Center => {}
                            BuildingType::WarBuilding => {},
                            BuildingType::WoodBuilding => player.wood += 20,
                            BuildingType::MetalBuilding => player.metal += 20,
                            BuildingType::FaithBuilding => player.faith += 20,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}