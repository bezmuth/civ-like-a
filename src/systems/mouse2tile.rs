use crate::game::MouseTilePos;
use amethyst::{
    core::{
        geometry::Plane,
        transform::Transform,
        math::{Point2, Vector2},
    },
    derive::SystemDesc,
    renderer::{Camera},
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage, ReadStorage, ReadExpect, WriteExpect},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

fn screen_to_tile(screen_x: f32, screen_y: f32) -> (i32, i32){ // Returns a coordinate within the 'tile grid'. Opposite of inital tile location lambda.
    let tile_x = ((screen_x / 32. + screen_y / 17.) /2.).round() as i32;   // using round to ensure correct tile is effected
    let tile_y = ((screen_y / 17. - (screen_x / 32.)) /2.).round() as i32; // without round the wrong tile will be returned, 
    return (tile_x, tile_y)                                                // try running without round to understand
}


#[derive(SystemDesc)]
pub struct M2TileSystem; // * mouse 2 tile system

impl<'s> System<'s> for M2TileSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, MouseTilePos>,
    );

    fn run(&mut self, (transforms, input, cameras, screen_dimensions, mut mouse_tile_pos): Self::SystemData) {
        if let Some(mouse_position) = input.mouse_position(){
            // using raycasting instead of just mouse_position because
            // mouse.position returns full desktop mouse position and does not
            // give coords relative to the game world resulting in complex math,
            // this is more processor intensive but it dont really matter cause
            // it runs fine with release optimization on the compiler.
            let camera_join = (&cameras, &transforms).join();
            let (camera, transform) = camera_join.last().unwrap(); // ! THIS ONLY WORKS CAUSE THERE IS ONLY 1 CAMERA!
            // Project a ray from the camera to the 0z axis
            let ray = camera.screen_ray(
                Point2::new(mouse_position.0, mouse_position.1),
                Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                &transform,
            );
            let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
            let mouse_world_position = ray.at_distance(distance);
            // Done: !ENSURE RAY IS NOT OVERLAPPING WITH ANY MENUS!
            let pos = screen_to_tile(mouse_world_position.x, mouse_world_position.y);
            mouse_tile_pos.pos.x = pos.0;
            mouse_tile_pos.pos.y = pos.1;

            // old method, does not work properly, keeping for documentation
            // purposes, this was similar to how I did it in my GODOT prototype.


            // let inputloc = input.mouse_position().unwrap();
            // let pos = screen_to_tile(inputloc.0 + Cpos.0, inputloc.1 + Cpos.1);
            // println!("{} {}", pos.0, pos.1);
        }
    }
}
