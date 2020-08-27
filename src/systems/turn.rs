use crate::game::{Player, PlayersInfo, Turn};

use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, WriteStorage, ReadExpect, WriteExpect}, ui::{UiEvent, UiText, UiEventType, UiFinder}, shred::{World, Write}, shrev::{ReaderId, EventChannel},
};



// TODO: implement full bar instead of just text, should be possible with ui library?

pub struct TurnSystem{
    event_reader: ReaderId<UiEvent>,
}

impl TurnSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let event_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader();
        Self { event_reader } // * gotta do this whenever trying to read events
    }
}

impl<'s> System<'s> for TurnSystem {
    type SystemData = (
        WriteExpect<'s, PlayersInfo>,
        WriteExpect<'s, Turn>,
        Write<'s, EventChannel<UiEvent>>,
        UiFinder<'s>,
    );



    fn run(&mut self, (mut playersinfo, mut turn, mut ui_events, ui_finder): Self::SystemData) {
        // let reader = self
        //     .event_reader
        //     .as_mut()
        //     .expect("TurnSystem::setup was not called correctly, try adding component with ::default()");

        for event in ui_events.read(&mut self.event_reader){ 
            if event.event_type == UiEventType::Click{
                
                let clicked = event.target.id();

                if clicked == ui_finder.find("Turn_button").unwrap().id(){
                    turn.num += 1;
                    playersinfo.current_player_num = turn.num % playersinfo.count;
                }
            }
        };
    }
}

