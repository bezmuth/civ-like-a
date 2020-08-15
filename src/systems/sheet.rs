use crate::game::{Tiles, Build, BuildingType, CurrentPlayer, Building, MouseTilePos};
use amethyst::{
    core::{
        geometry::Plane,
        transform::Transform,
        math::{Point2, Vector2},
    },
    derive::SystemDesc,
    renderer::{SpriteRender, Camera},
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage, ReadStorage, ReadExpect, WriteExpect, Entities},
    input::{InputHandler, StringBindings}, 
    winit::MouseButton,
    window::ScreenDimensions,
};




#[derive(SystemDesc)]
pub struct SheetSystem;

// TODO: split this up into multiple components?
impl<'s> System<'s> for SheetSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tiles>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, Build>,
        ReadExpect<'s, CurrentPlayer>,
        WriteStorage<'s, Building>,
        Entities<'s>,
        ReadExpect<'s, MouseTilePos>,
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (transforms, mut tiles, input, mut spriterenderers, cameras, screen_dimensions, mut build, currentplayer, mut buildings, entities, mouse_tile_pos): Self::SystemData) {
        // TODO: combine these 2 code blocks?

        
        
        let modi: Option<usize> = match build.mode { // checks the current building mode and returns the sprite to be used //TODO: ENSURE THIS HAS ALL THE BUILDING SPRITES
            BuildingType::FaithBuilding => Some(3 as usize),
            _ => None,
        };
        if modi.is_some(){
            for (tile, spriterender) in (&mut tiles, &mut spriterenderers).join() {
                if input.mouse_button_is_down(MouseButton::Left){
                    if (mouse_tile_pos.x == tile.x) && (mouse_tile_pos.y == tile.y) && tile.layer == 1{
                        spriterender.sprite_number = modi.unwrap();
                        tile.buildingtype = build.mode; // TODO: Check if this is redundant cause of the entity addition below
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
                    if (mouse_tile_pos.x == tile.x) && (mouse_tile_pos.y == tile.y) && tile.layer == 1{
                        spriterender.sprite_number = 2;
                        for (ent, building) in (&*entities, &mut buildings).join(){
                            if building.buildingtype == tile.buildingtype && building.playernum == currentplayer.playernum{
                                entities.delete(ent).expect("Could not delete this building, does it exist?");
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