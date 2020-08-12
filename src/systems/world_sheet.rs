use crate::game::{Tiles};
use amethyst::{
    core::{
        geometry::Plane,
        transform::Transform,
        math::{Point2, Vector2},
    },
    derive::SystemDesc,
    renderer::{SpriteRender, ActiveCamera, Camera},
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage, Entities, ReadStorage, ReadExpect},
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

impl<'s> System<'s> for SheetSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tiles>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, ActiveCamera>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        //WriteStorage<'s, TileEnts>
    );


    fn run(&mut self, (entities, transforms, mut tiles, input, mut spriterenderers, active_camera, cameras, screen_dimensions): Self::SystemData) {
        for (tile, spriterender) in (&mut tiles, &mut spriterenderers).join() {
            if input.mouse_button_is_down(MouseButton::Left){
                    let mut camera_join = (&cameras, &transforms).join();
                    if let Some(mouse_position) = input.mouse_position(){
                    // using raycasting instead of just mouse_position because mouse.position 
                    // returns full desktop mouse position and does not give coords relative 
                    // to the game world resulting in complex math, this is more processor intensive
                    // but it dont really matter cause this is ran very infrequently
                    if let Some((camera, camera_transform)) = active_camera
                        .entity
                        .and_then(|a| camera_join.get(a, &entities))
                        .or_else(|| camera_join.next())
                    {
                        // Project a ray from the camera to the 0z axis
                        let ray = camera.projection().screen_ray(
                            Point2::new(mouse_position.0, mouse_position.1),
                            Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                            camera_transform,
                        );
                        let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                        let mouse_world_position = ray.at_distance(distance);
                        
                        let pos = screen_to_tile(mouse_world_position.x, mouse_world_position.y);
                        // println!("{} {}", mouse_world_position.x, mouse_world_position.y);
                        if (pos.0 == tile.x) && (pos.1 == tile.y){
                            spriterender.sprite_number = 1;
                        }
                    }
                }
                // * old method, does not work properly, keeping for documentation purposes
                // let inputloc = input.mouse_position().unwrap();
                // let pos = screen_to_tile(inputloc.0 + Cpos.0, inputloc.1 + Cpos.1);
                // println!("{} {}", pos.0, pos.1);
                // if (pos.0 == tile.x) && (pos.1 == tile.y){
                //     spriterender.sprite_number = 1;
                // }
            }
        }
    }
}