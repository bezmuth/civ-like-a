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
        //TODO: fix debug toggle
        if input.action_is_down("debug").unwrap(){
            self.toggled = !self.toggled
        }
        if self.toggled{
            amethyst_imgui::with(|ui| {
                Window::new(im_str!("Debug"))
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(ui, || {
                    ui.text(format!("Current Player Number: {}/{}", playersinfo.current_player_num, playersinfo.count));
                    ui.text(format!("Current Tile Pos: ({}, {})", mouse_tile_pos.pos.x, mouse_tile_pos.pos.y));
                    ui.text(format!("Current Turn : {}", turn.num));
                    ui.text(format!("Mouse On UI element? : {}", onui.case));



                    ui.bullet_text(im_str!("cool"));
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