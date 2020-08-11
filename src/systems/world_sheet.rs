use crate::game::{Tiles};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct SheetSystem;

impl<'s> System<'s> for SheetSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Tiles>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, tiles, input): Self::SystemData) {
        for (_tiles, transform) in (&tiles, &mut transforms).join() {
            if let Some(mv_amount) = input.axis_value("updown") {
                let scaled_amount = 1.2 * mv_amount;
                let tile_y = transform.translation().y;
                transform.set_translation_y(tile_y + scaled_amount);
            } 
            if let Some(mv_amount) = input.axis_value("leftright") {
                let scaled_amount = 1.2 * mv_amount;
                let tile_x = transform.translation().x;
                transform.set_translation_x(tile_x + scaled_amount);
            }
        }
    }
}