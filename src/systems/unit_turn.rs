use crate::game::{PlayersInfo, Player, Building, TileType, Turn, Tiles, TilePos, Layer3, UnitStack, OutPos, Stat, Unit};

use amethyst::{
    renderer::{SpriteRender, resources::Tint,},
    ecs::prelude::{System, WriteStorage, ReadExpect, Join, Entities},
};


pub struct UnitTurnSystem{
    pub last_turn : i32,
}



impl<'s> System<'s> for UnitTurnSystem {
    type SystemData = (
        WriteStorage<'s, Building>,
        WriteStorage<'s, UnitStack>,
        WriteStorage<'s, OutPos>,
        ReadExpect<'s, Turn>,
        WriteStorage<'s, Tiles>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, TilePos>,
        WriteStorage<'s, Layer3>,
        ReadExpect<'s, PlayersInfo>,
        WriteStorage<'s, Unit>,
        WriteStorage<'s, Stat>,
        WriteStorage<'s, Player>,
        Entities<'s>,
        WriteStorage<'s, Tint>,
    );

    // One of the main issues that arose with this system is that if a unit
    // already exists in the output location of the building creating it then
    // the unit wont be created properly. This can be accounted for however it
    // is not within the scope of this project.
    fn run(&mut self, (mut buildings, mut unitstacks, mut outposs, turn, mut tiles, mut spriterenderers, mut tileposs, mut layer3, playersinfo, mut units, mut stats, mut players, mut entities, mut tints): Self::SystemData) {


        if turn.num > self.last_turn{
            // Allows all the units to move again
            for (_, stat) in (&mut units, &mut stats).join(){
                stat.exhausted = false;
            }

            // Creates the entities from the unit stack, setting their position to that of the building's outposs
            for (building, unitstack, outpos)  in (&mut buildings, &mut unitstacks, &mut outposs).join(){
                if building.playernum == playersinfo.current_player_num{
                    if let Some(future_unit_type) = unitstack.pop(){

                        let mut future_stats = Stat::default();
                        let mut future_pos = outpos.pos.clone();
                        let mut future_unit = Unit{unit_type : future_unit_type, playernum : building.playernum};
                        let mut current_player : Player;
                        let mut had_res = true;

                        // iteration that finds the current player
                        for current_player in (&mut players).join(){
                            if current_player.num == playersinfo.current_player_num {

                                match future_unit_type{
                                    TileType::Warrior => {
                                        future_stats = Stat{
                                            speed: 6.,
                                            health: 20.,
                                            attack: 8.,
                                            resistance: 0.1, // chance to not take damage
                                            multi_hit_chance: 0., // chance to multi hit
                                            multi_hit_amount: 0., // how many times to multi hit
                                            crit_chance: 0.2, //chance to do a random crit
                                            exhausted: false,
                                        };
                                        had_res = current_player.sub_wood(10); // checks if user had enough of the resources
                                    },
                                    TileType::Heavy => {
                                        future_stats = Stat{
                                            speed: 4.,
                                            health: 50.,
                                            attack: 6.,
                                            resistance: 0., // chance to not take damage
                                            multi_hit_chance: 0., // chance to multi hit
                                            multi_hit_amount: 0., // how many times to multi hit
                                            crit_chance: 0.1, //chance to do a random crit
                                            exhausted : false,
                                        };
                                        had_res = current_player.sub_both(20,25); // checks if user had enough of the resources
                                    },
                                    TileType::Monk => {
                                        future_stats = Stat{
                                            speed: 6.,
                                            health: 10.,
                                            attack: 4.,
                                            resistance: 0., // chance to not take damage
                                            multi_hit_chance: 0.8, // chance to multi hit
                                            multi_hit_amount: 3., // how many times to multi hit
                                            crit_chance: 0.3, //chance to do a random crit
                                            exhausted: false,
                                        };
                                        had_res = current_player.sub_both(5,30); // checks if user had enough of the resources
                                    }
                                    _ => future_stats = Stat::default()
                                }
                            }

                            if had_res{
                                entities
                                    .build_entity()
                                    .with(future_unit, &mut units)
                                    .with(future_pos, &mut tileposs)
                                    .with(future_stats, &mut stats)
                                    .build();
                                // iteration which sets the sprite render on the sheet
                                for (tile, spriterender, tilepos, tint, _) in (&mut tiles, &mut spriterenderers, &mut tileposs, &mut tints, &mut layer3).join() {
                                    if (& future_pos == tilepos) && tile.tile_type.is_none(){
                                        spriterender.sprite_number = future_unit_type as usize;
                                        tile.tile_type = Some(future_unit_type);
                                        tint.0.red = 1. - playersinfo.current_player_num as f32 * 0.4 % 1.;
                                        tint.0.blue = 1. - playersinfo.current_player_num as f32 * 0.5 % 1.;
                                        tint.0.green = 1. - playersinfo.current_player_num as f32 * 0.0 % 1.;
                                        tint.0.alpha = 0.1;
                                    }
                                }
                            } else {
                                unitstack.push(future_unit_type); // due to how we pop the unit stack, if the user does not have the currency we gotta put the unit back on the unit stack
                            }

                        }
                    }
                }
            }
            self.last_turn += 1;
        }
    }
}
