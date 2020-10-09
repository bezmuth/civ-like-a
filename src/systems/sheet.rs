use crate::game::{Tiles, Build, BuildingType, PlayersInfo, Building, MouseTilePos, Layer2};
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
impl<'s> System<'s> for SheetSystem {
    type SystemData = (
        WriteStorage<'s, Tiles>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        WriteExpect<'s, Build>,
        ReadExpect<'s, PlayersInfo>,
        WriteStorage<'s, Building>,
        Entities<'s>,
        ReadExpect<'s, MouseTilePos>,
        WriteStorage<'s, Layer2>, // can only join storages of the same read type
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (mut tiles, input, mut spriterenderers, mut build, playersinfo, mut buildings, entities, mouse_tile_pos, mut layer2): Self::SystemData) {
        // TODO: combine these 2 code blocks? and refactor this code is garbage. Most of the if statments should be replacable with a .join implementation in the object definition
        // TODO: convert some of these into functions cause they will be useful later (for enemies causing destruction)
        if build.mode.is_some(){
            if build.mode.unwrap() != BuildingType::Demolish{
                for (tile, spriterender, _) in (&mut tiles, &mut spriterenderers, &mut layer2).join() {
                    if input.mouse_button_is_down(MouseButton::Left){
                        if (mouse_tile_pos.x == tile.x) && (mouse_tile_pos.y == tile.y) && tile.buildingtype.is_none(){
                            spriterender.sprite_number = build.mode.unwrap() as usize;
                            tile.buildingtype = build.mode; 
                            entities // add an entity of the build.mode type to the world, allows for resource calc
                                .build_entity() 
                                .with(Building {buildingtype: build.mode.unwrap() , playernum: playersinfo.current_player_num, x: tile.x, y: tile.y}, &mut buildings)
                                .build();
                            if !input.action_is_down("extend").unwrap(){ // allows multiple buildings to be placed without pressing build a bunch of times
                                build.mode = None;
                            }
                        }
                    }
                }
            }
        }
        if build.mode.is_some(){
            if build.mode.unwrap() == BuildingType::Demolish{ // else if ensures build.mode cannot be set to None before interaction 
                for (tile, spriterender, ent, _) in (&mut tiles, &mut spriterenderers, &*entities, &mut layer2).join() {
                    if input.mouse_button_is_down(MouseButton::Left){
                        if (mouse_tile_pos.x == tile.x) && (mouse_tile_pos.y == tile.y){
                            for building in (&mut buildings).join(){
                                if tile.buildingtype.is_some() && building.buildingtype == tile.buildingtype.unwrap() && building.playernum == playersinfo.current_player_num && tile.x == building.x && tile.y == building.y{
                                    entities.delete(ent).expect("Could not delete this building, does it exist?");
                                    spriterender.sprite_number = 4; // blank sprite;
                                    tile.buildingtype = None;
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