use crate::game::{Tiles, Build, BuildingType, CurrentPlayer, Building, MouseTilePos, Layer2};
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
        ReadExpect<'s, CurrentPlayer>,
        WriteStorage<'s, Building>,
        Entities<'s>,
        ReadExpect<'s, MouseTilePos>,
        WriteStorage<'s, Layer2>, // can only join storages of the same read type
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (mut tiles, input, mut spriterenderers, mut build, currentplayer, mut buildings, entities, mouse_tile_pos, mut layer2): Self::SystemData) {
        // TODO: combine these 2 code blocks?

        
        
        let modi: Option<usize> = match build.mode { // checks the current building mode and returns the sprite to be used //TODO: ENSURE THIS HAS ALL THE BUILDING SPRITES
            BuildingType::Center => Some(3 as usize),
            BuildingType::WarBuilding => Some(3 as usize),
            BuildingType::WoodBuilding => Some(3 as usize),
            BuildingType::MetalBuilding => Some(3 as usize),
            BuildingType::FaithBuilding => Some(3 as usize),
            BuildingType::Demolish => Some(3 as usize),
            BuildingType::None => None,
        };
        if modi.is_some(){
            for (tile, spriterender, _) in (&mut tiles, &mut spriterenderers, &mut layer2).join() {
                if input.mouse_button_is_down(MouseButton::Left){
                    if (mouse_tile_pos.x == tile.x) && (mouse_tile_pos.y == tile.y) && tile.buildingtype == BuildingType::None{
                        spriterender.sprite_number = modi.unwrap();
                        tile.buildingtype = build.mode; 
                        entities // add an entity of the build.mode type to the world, allows for resource calc
                            .build_entity() 
                            .with(Building {buildingtype: build.mode , playernum: currentplayer.playernum}, &mut buildings)
                            .build();
                        if !input.action_is_down("extend").unwrap(){ // allows multiple buildings to be placed without pressing build a bunch of times
                            build.mode = BuildingType::None;
                        }
                    }
                }
            }
        }



        let runs: bool = match build.mode { // checks the current building mode and returns the sprite to be used
            BuildingType::Demolish => true,
            _ => false,
        };
        if runs{
            for (tile, spriterender) in (&mut tiles, &mut spriterenderers).join() {
                if input.mouse_button_is_down(MouseButton::Left){
                    if (mouse_tile_pos.x == tile.x) && (mouse_tile_pos.y == tile.y){
                        spriterender.sprite_number = 2;
                        for (ent, building) in (&*entities, &mut buildings).join(){
                            if building.buildingtype == tile.buildingtype && building.playernum == currentplayer.playernum{
                                entities.delete(ent).expect("Could not delete this building, does it exist?");
                                break; // TODO: see if there is another way to do this without break, without break all buildings of this type get detleted
                            }
                        }
                        tile.buildingtype = BuildingType::None;
                        if !input.action_is_down("extend").unwrap(){ // allows multiple buildings to be placed without pressing build a bunch of times
                            build.mode = BuildingType::None;
                        }
                    }
                }
            }
        }



    }
}