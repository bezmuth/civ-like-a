use crate::game::{CurrentPlayer, MouseTilePos};
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
        ReadExpect<'s, CurrentPlayer>,
        ReadExpect<'s, MouseTilePos>,
    );


    fn run(&mut self, (input, currentplayer, mouse_tile_pos): Self::SystemData) {
        //TODO: fix debug toggle
        if input.action_is_down("debug").unwrap(){
            self.toggled = !self.toggled
        }
        if self.toggled{
            amethyst_imgui::with(|ui| {
                Window::new(im_str!("Debug"))
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(ui, || {
                    ui.text(format!("Current Player Number: {}", currentplayer.playernum));
                    ui.text(format!("Current Tile Pos: ({}, {})", mouse_tile_pos.x, mouse_tile_pos.y));

                    ui.bullet_text(im_str!("cool"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
            });
        }
    }
}