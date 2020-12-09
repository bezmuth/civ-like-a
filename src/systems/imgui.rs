use crate::game::{MouseTilePos, OnUi, PlayersInfo, Turn};
use amethyst::{

    derive::SystemDesc,
    ecs::prelude::{Read, System, SystemData, ReadExpect},
    input::{InputHandler, StringBindings}, 
};

use amethyst_imgui::imgui::*;


#[derive(SystemDesc)]
pub struct Imgui{
    pub toggled : bool,
}

impl<'s> System<'s> for Imgui {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, PlayersInfo>,
        ReadExpect<'s, MouseTilePos>,
        ReadExpect<'s, Turn>,
        ReadExpect<'s, OnUi>,
    );


    fn run(&mut self, (input, playersinfo, mouse_tile_pos, turn, onui): Self::SystemData) {
        if input.action_is_down("debug").unwrap(){ // handles toggling the debug menu
            self.toggled = !self.toggled
        }
        if self.toggled{
            amethyst_imgui::with(|ui| {
                // The amethyst imgui library is just a bunch of C++ bindings so
                // the code is a bit funky, but it works well for what I want.
                // In fact imgui is so easy to work with and configure that I
                // wish I had used it instead of amethysts built in Ui library
                Window::new(im_str!("Debug"))
                .size([300.0, 110.0], Condition::FirstUseEver)
                    .build(ui, || {
                    // rust's built in format macro makes it easy to add
                    // variables into strings
                    ui.text(format!("Current Player Number: {}/{}", playersinfo.current_player_num, playersinfo.count));
                    ui.text(format!("Current Tile Pos: ({}, {})", mouse_tile_pos.pos.x, mouse_tile_pos.pos.y));
                    ui.text(format!("Current Turn : {}", turn.num));
                    ui.text(format!("Mouse On UI element? : {}", onui.case));



                    ui.separator();
                    let mouse_pos = input.mouse_position().unwrap();
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos.0, mouse_pos.1
                    ));
                });
            });
        }
    }
}
