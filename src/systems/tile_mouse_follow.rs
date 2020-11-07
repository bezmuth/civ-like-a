use crate::game::{Follower, MouseTilePos, OnUi, TilePos, Tiles};
use amethyst::{renderer::SpriteRender, core::Transform, derive::SystemDesc, ecs::{Join, ReadStorage, WriteStorage, prelude::{Read, System, SystemData, WriteExpect}}, input::{InputHandler, StringBindings}, shred::ReadExpect};

#[derive(SystemDesc)] 
pub struct TileMouseFollow; // * Detects if mouse is on UI elements, if so onui.case is set to true

impl<'s> System<'s> for TileMouseFollow {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadExpect<'s, MouseTilePos>,
        ReadStorage<'s, TilePos>,
        WriteStorage<'s, Follower>,
        ReadStorage<'s, Tiles>,
        WriteStorage<'s, SpriteRender>,

    );

    fn run(&mut self, (mut transforms, mouse_tile_pos, tile_positions, mut follower, tiles, mut spriterenders): Self::SystemData) {
        let mut cloned_transform = Transform::default();
        for (tile_pos, _, tile_transform) in (&tile_positions, &tiles, &transforms).join(){
            if tile_pos == &mouse_tile_pos.pos{
                cloned_transform = tile_transform.clone();
            }
        }
        if cloned_transform != Transform::default(){
            for (follower_transform, follower_spriterender, fol) in (&mut transforms, &mut spriterenders, &mut follower).join(){
                follower_spriterender.sprite_number = fol.kind as usize;
                *follower_transform = cloned_transform;
                break;
            }
        }
    }
    
}