use crate::game::{Follower, MouseTilePos, TilePos, Tiles};
use amethyst::{renderer::SpriteRender, core::Transform, derive::SystemDesc, ecs::{Join, ReadStorage, WriteStorage, prelude::{System, SystemData}}, shred::ReadExpect};

#[derive(SystemDesc)]
pub struct TileMouseFollow;

impl<'s> System<'s> for TileMouseFollow {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadExpect<'s, MouseTilePos>,
        ReadStorage<'s, TilePos>,
        WriteStorage<'s, Follower>,
        ReadStorage<'s, Tiles>,
        WriteStorage<'s, SpriteRender>,

    );
    // This system handles getting tile sprites to follow the mouse. Its pretty
    // simple and does not handle any of the logic (like checking to see if the
    // tile is valid) and leaves the other systems to handle that.
    fn run(&mut self, (mut transforms, mouse_tile_pos, tile_positions, mut follower, tiles, mut spriterenders): Self::SystemData) {
        let mut cloned_transform = Transform::default();
        for (tile_pos, _, tile_transform) in (&tile_positions, &tiles, &transforms).join(){
            if tile_pos == &mouse_tile_pos.pos{
                // The nice thing is we dont even need to calculate the
                // positions using isometric, instead we can just copy the
                // transform from an existing tile.
                cloned_transform = tile_transform.clone();
            }
        }
        for (follower_transform, follower_spriterender, fol) in (&mut transforms, &mut spriterenders, &mut follower).join(){
            follower_spriterender.sprite_number = fol.kind as usize;
            *follower_transform = cloned_transform;
            break;
        }
    }
}
