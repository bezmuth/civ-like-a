use crate::game::OnUi;
use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, WriteExpect},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct UIDetect;

impl<'s> System<'s> for UIDetect {
    type SystemData = (
        WriteExpect<'s, OnUi>,
        Read<'s, InputHandler<StringBindings>>,
    );
    // Detects if mouse is on UI elements, if so onui.case is set to true
    fn run(&mut self, (mut onui, input): Self::SystemData) {
        if let Some(mouse_position) = input.mouse_position(){
            if mouse_position.1 > 1070.{ // ! 1070 is when the Ui starts.
                onui.case = true;
            } else {
                onui.case = false;
            }
        }
    }
}
