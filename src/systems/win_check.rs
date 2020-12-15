use crate::game::{PlayersInfo, Building, TileType, WinState, Player, Turn};

use amethyst::{
    ecs::prelude::{Join, System, WriteStorage, ReadExpect, WriteExpect, Write},
};


pub struct WinCheckSystem;



impl<'s> System<'s> for WinCheckSystem {
    type SystemData = (
        WriteStorage<'s, Building>,
        ReadExpect<'s, Turn>,
        Write<'s, WinState>
    );


    fn run(&mut self, (buildings, turn, mut winstate): Self::SystemData) {
        if turn.num >= 2{
            let mut p1count = 0;
            let mut p2count = 0;
            for (build) in (buildings).join(){
                if build.playernum == 0 && build.tile_type == TileType::Center{
                    p1count += 1;
                } else if build.playernum == 1 && build.tile_type == TileType::Center{
                    p2count += 1;
                }
            }
            if p1count == 0{
                println!("p1 lost");
                winstate.case = true;
                winstate.num_winner = 2;
            } else if p2count == 0{
                println!("p2 lost");
                winstate.case = true;
                winstate.num_winner = 1;
            }
        }
    }
}
