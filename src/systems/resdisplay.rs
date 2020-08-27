use crate::game::{PlayersInfo, Player, Resbar};

use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadExpect, WriteExpect}, ui::UiText,
};



// TODO: implement full bar instead of just text, should be possible with ui library?

#[derive(SystemDesc)]
pub struct ResourceDispSystem;

impl<'s> System<'s> for ResourceDispSystem {
    type SystemData = (
        ReadExpect<'s, PlayersInfo>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, UiText>,
        WriteExpect<'s, Resbar>,
    );


    fn run(&mut self, (playersinfo, players, mut ui_text, resbar): Self::SystemData) {
        for player in (players).join(){
            if player.num == playersinfo.current_player_num{
                if let Some(text) = ui_text.get_mut(resbar.top) {
                    text.text = format!("Wood: {}, Metal: {}, Faith: {}", player.wood, player.metal, player.faith);
                }
            }
        }
    }
}

