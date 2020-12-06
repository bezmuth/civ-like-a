use crate::game::{PlayersInfo, Player, Building, TileType, Turn};

use amethyst::{
    ecs::prelude::{Join, System, WriteStorage, ReadExpect},
};


pub struct ResourceCalcSystem{
    pub last_turn : i32,
}



impl<'s> System<'s> for ResourceCalcSystem {
    type SystemData = (
        ReadExpect<'s, PlayersInfo>,
        WriteStorage<'s, Building>,
        WriteStorage<'s, Player>,
        ReadExpect<'s, Turn>,
    );


    fn run(&mut self, (playersinfo, buildings, mut players, turn): Self::SystemData) {
        // TODO: integrate with turn system 


        if turn.num > self.last_turn{ // checks if a turn has passed, if it has add resources to current player
            for building in (buildings).join(){
                if building.playernum == playersinfo.current_player_num {
                    for player in (&mut players).join(){
                        if player.num == playersinfo.current_player_num {
                            match building.tile_type { // These seem pretty balanced
                                TileType::Center => { player.wood += 5; player.metal += 5; player.science += 5},
                                TileType::Barrack => {},
                                TileType::WoodBuilding => player.wood += 10,
                                TileType::MetalBuilding => player.metal += 10,
                                TileType::ScienceBuilding => player.science += 20,
                                _ => {}
                            }
                        }
                    }
                }
            }
            self.last_turn += 1;
        }
    }
}
