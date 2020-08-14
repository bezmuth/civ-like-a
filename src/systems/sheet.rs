use crate::game::{Tiles, Build, BuildingType, CurrentPlayer, Building};
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


fn screen_to_tile(screen_x: f32, screen_y: f32) -> (i32, i32){ // * Returns a coordinate within the 'tile grid'. Opposite of inital tile location maths.
    let tile_x = ((screen_x / 32. + screen_y / 17.) /2.).round() as i32; // using round to ensure correct tile is effected
    let tile_y = ((screen_y / 17. - (screen_x / 32.)) /2.).round() as i32; // without round the wrong tile will be clicked, 
    return (tile_x, tile_y)                                              // try running without round to understand
}

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
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (transforms, mut tiles, input, mut spriterenderers, cameras, screen_dimensions, mut build, currentplayer, mut buildings, entities): Self::SystemData) {
        let modi: Option<usize> = match build.mode { // checks the current building mode and returns the sprite to be used //TODO: ENSURE THIS HAS ALL THE BUILDING SPRITES
            BuildingType::FaithBuilding => Some(3 as usize),
            _ => None,
        };
        if modi.is_some(){
            for (tile, spriterender) in (&mut tiles, &mut spriterenderers).join() {
                if input.mouse_button_is_down(MouseButton::Left){
                    if let Some(mouse_position) = input.mouse_position(){
                        let camera_join = (&cameras, &transforms).join();
                        // using raycasting instead of just mouse_position because mouse.position 
                        // returns full desktop mouse position and does not give coords relative 
                        // to the game world resulting in complex math, this is more processor intensive
                        // but it dont really matter cause this is ran very infrequently
                        let (camera, transform) = camera_join.last().unwrap(); // ! THIS ONLY WORKS CAUSE THERE IS ONLY 1 CAMERA!
                        // Project a ray from the camera to the 0z axis
                        let ray = camera.projection().screen_ray(
                            Point2::new(mouse_position.0, mouse_position.1),
                            Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                            &transform,
                        );
                        let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                        let mouse_world_position = ray.at_distance(distance);
                        // todo: !ENSURE RAY IS NOT OVERLAPPING WITH ANY MENUS!
                        let pos = screen_to_tile(mouse_world_position.x, mouse_world_position.y);
                        // println!("{} {}", mouse_world_position.x, mouse_world_position.y);
                
                        if (pos.0 == tile.x) && (pos.1 == tile.y) && tile.layer == 1{
                            spriterender.sprite_number = modi.unwrap();
                            tile.buildingtype = build.mode; // TODO: Check if this is redundant cause of the entity addition below
                            entities // add an entity of the build.mode type to the world, allows for resource calc
                                .build_entity() //TODO: implement demolish, might require a rewrite
                                .with(Building {buildingtype: build.mode , playernum: currentplayer.playernum}, &mut buildings)
                                .build();
                            if !input.action_is_down("extend").unwrap(){ // allows multiple buildings to be placed without pressing build a bunch of times
                                build.mode = BuildingType::None;
                            }
                        }
                        // * old method, does not work properly, keeping for documentation purposes
                        // let inputloc = input.mouse_position().unwrap();
                        // let pos = screen_to_tile(inputloc.0 + Cpos.0, inputloc.1 + Cpos.1);
                        // println!("{} {}", pos.0, pos.1);
                    }
    
                }
            }
        }
    }
}