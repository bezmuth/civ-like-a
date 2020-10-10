use crate::game::{Build, BuildingType};
use amethyst::{
    ecs::{prelude::{System, WriteExpect, Write}, WriteStorage},
    ui::{UiFinder, UiEvent, UiEventType}, 
    shrev::{ReaderId, EventChannel}, 
    shred::World, core::{HiddenPropagate},

};


#[derive(Default)] // allows ::default() to be called, forcing setup to be run
pub struct BuildSystem{
    event_reader: Option<ReaderId<UiEvent>>,
    build_toggle: bool,
    last_mode: bool,
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        WriteExpect<'s, Build>,
        Write<'s, EventChannel<UiEvent>>,
        UiFinder<'s>,
        WriteStorage<'s, HiddenPropagate> // * hides widget and all children
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (mut build, events, ui_finder, mut hiddenpropagates): Self::SystemData) {
        let reader = self
            .event_reader
            .as_mut()
            .expect("BuildSystem::setup was not called correctly, try adding component with ::default()");


        for event in events.read(reader) {
            match event.event_type{
                UiEventType::Click => { // Todo: block sheet input if clicking on a ui element
                    
                    let clicked = event.target.id();
                    
                    // * tried to use match but it didnt work? Got variable not used warnings?
                    if clicked == ui_finder.find("Build_button").unwrap().id(){ 
                        self.build_toggle = !self.build_toggle;
                    } else if clicked == ui_finder.find("Demolish_button").unwrap().id() {
                        build.mode = Some(BuildingType::Demolish);
                    } else if clicked == ui_finder.find("Center_button").unwrap().id() {
                        build.mode = Some(BuildingType::Center)
                    } else if clicked == ui_finder.find("Barrack_button").unwrap().id() {
                        build.mode = Some(BuildingType::Barrack)
                    } else if clicked == ui_finder.find("WoodBuilding_button").unwrap().id() {
                        build.mode = Some(BuildingType::WoodBuilding)
                    } else if clicked == ui_finder.find("MetalBuilding_button").unwrap().id() {
                        build.mode = Some(BuildingType::MetalBuilding)
                    } else if clicked == ui_finder.find("ScienceBuilding_button").unwrap().id() {
                        build.mode = Some(BuildingType::ScienceBuilding)
                    }
                    
                },
                _ => {}
            }
        }


        if let Some(build_menu) = ui_finder.find("build_menu"){
            if self.build_toggle{
                if !self.last_mode{
                    let _  = hiddenpropagates.insert(build_menu, HiddenPropagate::new()).unwrap();
                    self.last_mode = true;
                }
            } 
            else {
                if self.last_mode{
                    let _  = hiddenpropagates.remove(build_menu).unwrap();
                    self.last_mode = false
                }
            }
        }


    }

    fn setup(&mut self, world: &mut World) {
        self.event_reader = Some(world.fetch_mut::<EventChannel<UiEvent>>().register_reader());
    }
    
}