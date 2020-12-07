use crate::game::{Build, TileType, PlayersInfo, Player, Turn};
use amethyst::{core::{HiddenPropagate}, ecs::{Join, WriteStorage, prelude::{System, WriteExpect, Write, ReadExpect}}, shred::World, shrev::{ReaderId, EventChannel}, ui::{UiFinder, UiEvent, UiEventType}};


#[derive(Default)] // allows ::default() to be called, forcing setup to be run
pub struct BuildSystem{
    event_reader: Option<ReaderId<UiEvent>>,
    run: bool,
    centered_turn: i32, // ensure only 1 center can be created per turn
}

impl<'s> System<'s> for BuildSystem {
    type SystemData = (
        WriteExpect<'s, Build>,
        Write<'s, EventChannel<UiEvent>>,
        UiFinder<'s>,
        WriteStorage<'s, HiddenPropagate>, // * hides widget and all children
        ReadExpect<'s, PlayersInfo>,
        WriteStorage<'s, Player>,
        ReadExpect<'s, Turn>,
        //WriteStorage<'s, TileEnts>
    );

    fn run(&mut self, (mut build, events, ui_finder, mut hidden_propagates, playersinfo, mut players, turn): Self::SystemData) {
        let reader = self
            .event_reader
            .as_mut()
            .expect("BuildSystem::setup was not called correctly, try adding component with ::default()!");
        if !self.run{
            if let Some(build_menu) = ui_finder.find("build_menu"){
                let _  = hidden_propagates.insert(build_menu, HiddenPropagate::new()).unwrap();
                self.run = true;
                self.centered_turn = -1;
            }
        } else {
            if let Some(build_menu) = ui_finder.find("build_menu"){

                for event in events.read(reader) {
                    match event.event_type{
                        UiEventType::Click => { // Todo: block sheet input if clicking on a ui element
                            let clicked = event.target.id();
                            // * tried to use match but it didnt work? Got variable not used warnings?
                            if clicked == ui_finder.find("Build_button").unwrap().id(){ 
                                if !hidden_propagates.contains(build_menu){ // todo: adapt this to a hidden_propagates.contains
                                    let _  = hidden_propagates.insert(build_menu, HiddenPropagate::new()).unwrap();
                                }else if hidden_propagates.contains(build_menu){
                                    let _  = hidden_propagates.remove(build_menu).unwrap();
                                }
                            } else { // TODO: some sort of indicator that shows how much each building will cost
                                if clicked == ui_finder.find("Demolish_button").unwrap().id() {
                                    build.mode = Some(TileType::Demolish);
                                } else if clicked == ui_finder.find("Center_button").unwrap().id() {
                                    if self.centered_turn < turn.num{ // ensures only one center per turn
                                        build.mode = Some(TileType::Center);
                                        self.centered_turn = turn.num
                                    }
                                } else if clicked == ui_finder.find("Barrack_button").unwrap().id() {
                                    build.mode = Some(TileType::Barrack);
                                } else if clicked == ui_finder.find("WoodBuilding_button").unwrap().id() {
                                    build.mode = Some(TileType::WoodBuilding);
                                } else if clicked == ui_finder.find("MetalBuilding_button").unwrap().id() {
                                    build.mode = Some(TileType::MetalBuilding);
                                } else if clicked == ui_finder.find("ScienceBuilding_button").unwrap().id() {
                                    build.mode = Some(TileType::ScienceBuilding)
                                }
                                let _  = hidden_propagates.insert(build_menu, HiddenPropagate::new()).unwrap();
                            }

                        },
                        _ => {}
                    }
                }
            }
        }



    }

    fn setup(&mut self, world: &mut World) {
        self.event_reader = Some(world.fetch_mut::<EventChannel<UiEvent>>().register_reader());
    }
    
}
