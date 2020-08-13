use crate::game::{Build, BuildingType};
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, WriteExpect},
    input::{InputHandler, StringBindings}, 
};

#[derive(SystemDesc)]
pub struct BuildSystem;

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        WriteExpect<'s, Build>,
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (input, mut build): Self::SystemData) {
        if input.action_is_down("build").unwrap(){
            build.mode = BuildingType::FaithBuilding;
        }
    }
}