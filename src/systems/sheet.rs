use crate::game::OutPos;
use crate::game::{Build, Building, TileType, Layer2, MouseTilePos, PlayersInfo, TilePos, Tiles, UnitStack};
use amethyst::{
    derive::SystemDesc,
    renderer::{SpriteRender},
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
        WriteStorage<'s, OutPos>
        //WriteStorage<'s, TileEnts>
    );
    // handles building to the sheet, should maybe be called build_sheet?
    fn run(&mut self, (mut tiles, mut tileposs, input, mut spriterenderers, mut build, playersinfo, mut buildings, entities, mouse_tile_pos, mut layer2, mut unitstacks, mut outposes): Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left){ // * this entire system only runs if the left mouse button is pressed
            // TODO: combine these 2 code blocks? and refactor this code is garbage. Most of the if statments should be replacable with a .join implementation in the object definition
            // TODO: convert some of these into functions cause they will be useful later (for enemies causing destruction)
            if let Some(build_mode) = build.mode{
                // build code
                if build_mode != TileType::Demolish{
                    // * This is split into two parts as one "cannot borrow `tileposs` as mutable more than once at a time" (ie in the for loop then in the entity creation)
                    let mut future_building: Option<Building> = None;
                    let mut future_pos: Option<TilePos> = None;
                    let mut future_outpos: Option<OutPos> = None;

                    // Instead of drawing to the screen, most rendering is handelled by setting the spriterender of a tile on a layer to a certain value
                    // This is a method of programming to an interface instead of directly creating a new spritrender and transform each time I want to create a new
                    // building, I instead change the value of a tile's spriterender.

                    // iteration which sets the sprite render, finds the tile position, 
                    for (tile, spriterender, tilepos, _) in (&mut tiles, &mut spriterenderers, &mut tileposs, &mut layer2).join() {
                        if (& mouse_tile_pos.pos == tilepos) && tile.tile_type.is_none(){
                            spriterender.sprite_number = build.mode.unwrap() as usize;
                            tile.tile_type = Some(build_mode);
                            if tilepos.x == 49{ // checks if the output position will be out of range of the tilemap
                                // TODO update to support dynamic pull of tilemap size?
                                future_outpos = Some(OutPos{ pos : TilePos{x:tilepos.x - 1 , y:tilepos.y}});
                            } else {
                                future_outpos = Some(OutPos{ pos : TilePos{x:tilepos.x - 1 , y:tilepos.y}});
                            }
                            future_building = Some(Building {tile_type: build_mode , playernum: playersinfo.current_player_num});
                            future_pos = Some(tilepos.clone());

                            if !input.action_is_down("extend").unwrap(){ // allows multiple buildings to be placed without pressing build a bunch of times
                                build.mode = None;
                            }
                        }
                    }


                    // actual creation of the entity
                    if let Some(future_build) = future_building{
                        if future_build.tile_type == TileType::Barrack{
                            entities // add an entity of the build.mode type to the world, allows for resource calc
                            .build_entity() 
                            .with(future_build, &mut buildings)
                            .with(future_pos.unwrap(), &mut tileposs)
                            .with(UnitStack::new(), &mut unitstacks)
                            .with(future_outpos.unwrap(), &mut outposes)
                            .build(); // todo: figure out how to add a component to a entity after the entity has been created, this would make checking if a tile had a building on it really simple (because it would be irrelavent)
                        } else {
                            entities // add an entity of the build.mode type to the world, allows for resource calc
                            .build_entity()
                            .with(future_build, &mut buildings)
                            .with(future_pos.unwrap(), &mut tileposs)
                            .build(); // todo: figure out how to add a component to a entity after the entity has been created, this would make checking if a tile had a building on it really simple (because it would be irrelavent)
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
                                    spriterender.sprite_number = 4; // blank sprite;
                                    tile.tile_type = None;
                                    break; // TODO: see if there is another way to do this without break, without break all buildings of this type get detleted
                                }
                            }
                            if !input.action_is_down("extend").unwrap(){ // allows multiple buildings to be placed without pressing build a bunch of times
                                build.mode = None;
                            }
                        }
                    }
                }
            }


        }
    }
}
