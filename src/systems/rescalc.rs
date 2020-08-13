use crate::game::{CurrentPlayer, Player, Building, BuildingType};

use amethyst::{
    core::{
        Time
    },
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage, ReadExpect},
    input::{InputHandler, StringBindings}, 
};

#[derive(SystemDesc)]
pub struct ResourceCalcSystem;

impl<'s> System<'s> for ResourceCalcSystem {
    type SystemData = (
        ReadExpect<'s, CurrentPlayer>,
        Read<'s, Time>,
        WriteStorage<'s, Building>,
        WriteStorage<'s, Player>,
    );


    fn run(&mut self, (currentplayer, time, Buildings, mut Players): Self::SystemData) {
        for building in (Buildings).join(){
            if building.playernum == currentplayer.playernum{
                for player in (&mut Players).join(){
                    if player.num == currentplayer.playernum {
                        println!("Wood: {} Metal: {} Faith: {}", player.wood, player.metal, player.faith);
                        match building.buildingtype { // TODO: Ensure these are balanced
                            BuildingType::Center => {}
                            BuildingType::WarBuilding => {},
                            BuildingType::WoodBuilding => player.wood = 200,
                            BuildingType::MetalBuilding => player.metal = 200,
                            BuildingType::FaithBuilding => player.faith = 200,
                            BuildingType::None => {}
                        }
                    }
                }
            }
        }
    }
}