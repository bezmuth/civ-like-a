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
            if mouse_position.1 > 1070.{ // ! 1070 is when the Ui starts. I ran
                // into an interesting race condition due to my use of a static
                // value for where the Ui restarts. Sometimes if my mouse is
                // focused on my 2nd monitor whilst compiling the game would
                // launch on my second monitor, for some reason (despite the
                // window not being resizable) my window manager resizes the
                // window moving some of the Ui elements, making this static
                // number invalid.
                onui.case = true;
            } else {
                onui.case = false;
            }
        }
    }
}
