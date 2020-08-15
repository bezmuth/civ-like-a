use crate::game::{Build, BuildingType};
use amethyst::{
    ecs::{Entities, prelude::{Read, System, WriteExpect, Write,WriteStorage}, Entity},
    input::{InputHandler, StringBindings}, 
    ui::{UiFinder, UiEvent, UiEventType, UiButton, UiText}, 
    shrev::{ReaderId, EventChannel}, 
    shred::World,

};


#[derive(Default)] // allows ::default() to be called, forcing setup to be run
pub struct BuildSystem{
    event_reader: Option<ReaderId<UiEvent>>
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        WriteExpect<'s, Build>,
        Entities<'s>,
        Write<'s, EventChannel<UiEvent>>,
        UiFinder<'s>,
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (input, mut build, entities, events, ui_finder): Self::SystemData) {
        let reader = self
            .event_reader
            .as_mut()
            .expect("BuildSystem::setup was not called correctly, try adding component with ::defauly()");

        for event in events.read(reader) {
            match event.event_type{
                UiEventType::Click => {
                    let clicked = event.target.id();
                    // TODO: implement full build menu with all building types
                    let build_button = ui_finder.find("Build_button").expect("Either: build_menu.ron was not loaded or the id was wrong").id();
                    let demolish_button = ui_finder.find("Demolish_button").expect("Either: build_menu.ron was not loaded or the id was wrong").id();
                    
                    // * tried to use match but it didnt work? Got variable not used warnings?
                    if clicked == build_button{ 
                        println!("Build!");
                        build.mode = BuildingType::FaithBuilding;
                    } else if clicked == demolish_button {
                        println!("Demolish!");
                        build.mode = BuildingType::Demolish;
                    }


                },
                _ => {}
            }
        }

    }

    fn setup(&mut self, world: &mut World) {
        self.event_reader = Some(world.fetch_mut::<EventChannel<UiEvent>>().register_reader());
    }
    
}