use crate::game::OutPos;
use crate::game::{Build, Building, TileType, Layer2, Layer1, MouseTilePos, PlayersInfo, TilePos, Tiles, UnitStack, Follower, Player};
use amethyst::{
    derive::SystemDesc,
    renderer::{SpriteRender, resources::Tint, palette::Srgba,},
    ecs::{prelude::{Join, Read, System, SystemData, WriteStorage, ReadExpect, WriteExpect, Entities}},
    input::{InputHandler, StringBindings}, 
    winit::MouseButton,
};




#[derive(SystemDesc)]
pub struct SheetSystem;

// TODO: split this up into multiple components?
impl<'s> System<'s> for SheetSystem { // handles user interaction with building to the sheet

    type SystemData = (
        WriteStorage<'s, Tiles>,
        WriteStorage<'s, TilePos>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        WriteExpect<'s, Build>,
        ReadExpect<'s, PlayersInfo>,
        WriteStorage<'s, Building>,
        Entities<'s>,
        WriteExpect<'s, MouseTilePos>,
        WriteStorage<'s, Layer2>, // can only join storages of the same read type
        WriteStorage<'s, UnitStack>,
        WriteStorage<'s, OutPos>,
        WriteStorage<'s, Follower>,
        WriteStorage<'s, Layer1>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Tint>,
        //WriteStorage<'s, TileEnts>
    );
    // handles building to the sheet, should maybe be called build_sheet? This
    // is a bit of a monalith of a system. All the functionality is related,
    // however its hard to manage and can be dificult to understand what the
    // code actually means some of the time even with heavy commenting. Its also
    // one of the few systems that directly communicates with other systems
    // using build.mode 
    fn run(&mut self, (mut tiles, mut tileposs, input, mut spriterenderers, mut build, playersinfo, mut buildings, entities, mouse_tile_pos, mut layer2, mut unitstacks, mut outposes, mut follower, mut layer1, mut players, mut tints): Self::SystemData) {
        if let Some(build_mode) = build.mode{

            let mut placeable = false;

            // iteration which checks if a tile is placeable upon
            for (tile, tilepos, _) in (& tiles, & tileposs, & layer1).join(){
                if let Some(current_tile_type) = tile.tile_type{
                    if (& mouse_tile_pos.pos == tilepos) {
                        match build_mode{
                            TileType::Center => placeable = current_tile_type == TileType::Ruins, // doing a comparison instead of an entire if statement cause I'm cool
                            TileType::Barrack => {
                                // We need to check that we are within range of a city centre
                                // We do this by checking if a city centre is in a 4 by 4 range of the mouse by pythag.
                                if current_tile_type == TileType::Grass{
                                    for (building, tilepos2) in (&buildings, &tileposs).join(){
                                        if building.tile_type == TileType::Center{
                                            if ((mouse_tile_pos.pos.x - tilepos2.x).pow(2) as f32 + (mouse_tile_pos.pos.y - tilepos2.y).pow(2) as f32).sqrt() <= 4.{
                                                placeable = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            TileType::WoodBuilding => placeable = current_tile_type == TileType::Forest,
                            TileType::MetalBuilding => placeable = current_tile_type == TileType::Mountains,
                            _ => placeable = false,
                        }
                    }
                }
            }

            if placeable{ // this sets the follower when a tile is placeable upon
                for fol in (&mut follower).join(){
                    if build_mode != TileType::Demolish{
                        fol.kind = build_mode;
                    }
                };
            } else {
                for fol in (&mut follower).join(){
                    if build_mode != TileType::Demolish{
                        fol.kind = TileType::Empty;
                    }
                };
            }




            if input.mouse_button_is_down(MouseButton::Left){ // this entire block only runs if the left mouse button is pressed
                // TODO: combine these 2 code blocks? and refactor this code is kinda painful. Most of the if statments should be replacable with a .join implementation in the object definition
                // TODO: convert some of these into functions cause they might be useful later (for enemies causing destruction?)
                // build code
                if build_mode != TileType::Demolish{
                    // * This is split into two parts as one "cannot borrow `tileposs` as mutable more than once at a time" (ie in the for loop then in the entity creation)
                    let mut future_building: Building = Building {tile_type: build_mode , playernum: playersinfo.current_player_num};
                    let mut future_pos: Option<TilePos> = None;
                    let mut future_outpos: Option<OutPos> = None;


                    if placeable {
                        //creation of the entity
                        let mut had_res = true; // check if the player had the resources to do this operation
                        // iteration that attempts to charge player
                        for player in (&mut players).join(){
                            if playersinfo.current_player_num == player.num{
                                match future_building.tile_type{
                                    TileType::Barrack => {had_res = player.sub_both(30, 10)},
                                    TileType::WoodBuilding => {had_res = player.sub_wood(20)},
                                    TileType::MetalBuilding =>{had_res = player.sub_both(10, 45)},
                                    _ => {}
                                }
                            }
                        }

                        if had_res{
                            // iteration which sets the sprite render, finds the tile position,

                            // Instead of drawing to the screen, most rendering is
                            // handelled by setting the spriterender of a tile on a
                            // layer to a certain value This is a method of programming
                            // to an interface instead of directly creating a new
                            // spritrender and transform each time I want to create a
                            // new building, I instead change the value of a tile's
                            // spriterender.
                            for (tile, spriterender, tilepos,tint, _) in (&mut tiles, &mut spriterenderers, &mut tileposs, &mut tints, &mut layer2).join() {
                                if (& mouse_tile_pos.pos == tilepos) && tile.tile_type.is_none(){
                                    spriterender.sprite_number = build.mode.unwrap() as usize;
                                    tile.tile_type = Some(build_mode);
                                    //tint = &mut Tint(Srgba::new(playersinfo.current_player_num as f32 * 0.5 % 1., playersinfo.current_player_num as f32 * 0.05 % 1., playersinfo.current_player_num as f32 * 0.3 % 1., 1.0));
                                    tint.0.red = 1. - playersinfo.current_player_num as f32 * 0.4 % 1.;
                                    tint.0.blue = 1. - playersinfo.current_player_num as f32 * 0.5 % 1.;
                                    tint.0.green = 1. - playersinfo.current_player_num as f32 * 0.0 % 1.;
                                    tint.0.alpha = 0.1;
                                    if tilepos.x == 0{ // checks if the output position will be out of range of the tilemap
                                        // TODO update to support dynamic pull of tilemap size?
                                        future_outpos = Some(OutPos{ pos : TilePos{x:tilepos.x + 1 , y:tilepos.y}});
                                    } else {
                                        future_outpos = Some(OutPos{ pos : TilePos{x:tilepos.x - 1 , y:tilepos.y}});
                                    }
                                    future_pos = Some(tilepos.clone());

                                }
                            }


                            if future_building.tile_type == TileType::Barrack{
                                entities // add an entity of the build.mode type to the world, allows for resource calc
                                    .build_entity() 
                                    .with(future_building, &mut buildings)
                                    .with(future_pos.unwrap(), &mut tileposs)
                                    .with(UnitStack::new(), &mut unitstacks)
                                    .with(future_outpos.unwrap(), &mut outposes)
                                    .build();
                            } else {
                                entities // add an entity of the build.mode type to the world, allows for resource calc
                                    .build_entity()
                                    .with(future_building, &mut buildings)
                                    .with(future_pos.unwrap(), &mut tileposs)
                                    .build();
                            }
                        }
                    }
                }

                // demolish code
                if build_mode == TileType::Demolish{ // else if ensures build.mode cannot be set to None before interaction
                    for (tile, spriterender, pos, _) in (&mut tiles, &mut spriterenderers,  &mut tileposs, &mut layer2).join() {
                        if & mouse_tile_pos.pos == pos{
                            for (building, ent) in (&mut buildings, &*entities,).join(){
                                if tile.tile_type.is_some() && building.playernum == playersinfo.current_player_num {
                                    entities.delete(ent).expect("Could not delete this building, does it exist?");
                                    spriterender.sprite_number = TileType::Empty as usize; // blank sprite;
                                    tile.tile_type = None;
                                    break; // TODO: see if there is another way to do this without break, without break all buildings of this type get detleted
                                }
                            }

                        }
                    }
                }
                // This code block became problematic once buildings required resources to create
                // if !input.action_is_down("extend").unwrap(){ // allows multiple buildings to be placed without pressing build a bunch of times
                //     build.mode = None;
                //     for fol in (&mut follower).join(){
                //         fol.kind = TileType::Empty;
                //     };
                // }
                build.mode = None;
                for fol in (&mut follower).join(){
                    fol.kind = TileType::Empty;
                };
            }
        }
    }
}
