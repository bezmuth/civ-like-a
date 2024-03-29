use crate::game::{Build, Building, TileType, MouseTilePos, OnUi, PlayersInfo, TilePos, Follower, UnitStack, Unit, OutPos};
use TileType::Empty;
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::ecs::Storage;
use amethyst::ecs::storage;

use amethyst::{core::HiddenPropagate, ui::{UiEventType, UiText}, ecs::WriteStorage, ecs::prelude::{System, ReadStorage, ReadExpect, SystemData, Join, WriteExpect}, input::{InputHandler, StringBindings}, shred::Read, shred::World, shred::Write, shrev::EventChannel, shrev::ReaderId, ui::UiEvent, ui::UiFinder, winit::MouseButton};

//fn stack_ui_update(stack : &mut UnitStack, ui_finder : UiFinder, mut ui_text : Storage<'_, UiText, amethyst::shred::FetchMut<'_, storage::MaskedStorage<UiText>>, >){ // * this function updates the stack ui elements, its a bit a gross function but I cannot iterate over Ui elements from arbritary indexes

    // ! HERES THE THING, AMETHYST DOES NOT ALLOW PREFABED BUTTON TEXT TO BE ACCESSED, SO THE UI METHOD OF STACK DISPLAY IS IMPOSSIBLE

//}


pub struct BuildingInteractSystem{
    event_reader: ReaderId<UiEvent>,
    first_run: bool,
    focused: bool,
    location_mode: bool, // location mode is when the user tries to chang the output location of a unit producing building
    focused_ent: Option<Entity>,
}

impl BuildingInteractSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let event_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader(); // Do this whenever trying to read events
        Self {  event_reader, first_run:true , focused: false, location_mode: false, focused_ent : None, }
    }
}

impl<'s> System<'s> for BuildingInteractSystem {
    type SystemData = (
        ReadStorage<'s, TilePos>,
        ReadExpect<'s, MouseTilePos>,
        UiFinder<'s>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, HiddenPropagate>, // * hides widget and all children
        ReadExpect<'s, PlayersInfo>,
        ReadStorage<'s, Building>,
        Write<'s, EventChannel<UiEvent>>,
        WriteExpect<'s, Build>,
        ReadExpect<'s, OnUi>,
        WriteStorage<'s, Follower>,
        WriteStorage<'s, UnitStack>,
        Entities<'s>,
        WriteStorage<'s, OutPos>
        );
        // * quick note on wanting to do a small menu above the tile, the
        // location of a Ui element (a UiTransform) cannot be written to
        // therefore I either have to create a static menu, for instance a menu
        // that replaces the lower panel like aoe2 OR I could move the camera
        // so the menu apears above the tile

        // In the end I decided on drawing over the bottom panel, aoe2 style
    fn run(&mut self, (tileposs, mouse_tile_pos, ui_finder, input, mut hidden_propagates, playersinfo, buildings, events, build, onui, mut follower, mut unitstacks, entities, mut outposs): Self::SystemData) {
        if !self.location_mode{ // if operating normally
            if self.first_run{ // * runs on first execute to ensure the barracks menu is hidden
                if let Some(interact_menu) = ui_finder.find("barracks_menu"){
                    let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                    self.first_run = false;
                }
            }

            // when we leave location mode, return the follower tile to that of
            // an empty sprite This is kinda haky but I didnt see the point of
            // adding a "just focused" variable, instead we check if the focused
            // ent is set to something, if thats the case then we must have just
            // left focused mode.
            for fol in (&mut follower).join(){
                if fol.kind == TileType::Location && self.focused_ent.is_some(){
                    self.focused_ent = None;
                    fol.kind = TileType::Empty
                }
            }

            if !onui.case{ // first check that the ui is not being interacted with
                if input.mouse_button_is_down(MouseButton::Left) && (build.mode.is_none() || build.mode.unwrap() != TileType::Demolish) && !self.focused{  // dont interact with buildings when in build mode
                    for (building, building_tile_pos, ent) in (&buildings, &tileposs, &entities).join(){
                        if (& mouse_tile_pos.pos == building_tile_pos) //todo: the "this tile has been clicked" should really be its own function/event. Can I make my own events in amethyst?
                        && building.playernum == playersinfo.current_player_num
                        {
                            if building.tile_type == TileType::Barrack{ // if clicking on barrack
                                if let (Some(interact_menu), Some(lower_panel)) = (ui_finder.find("barracks_menu"), ui_finder.find("lower_panel")){
                                    if hidden_propagates.contains(interact_menu){
                                        let _  = hidden_propagates.remove(interact_menu).unwrap(); // todo: implement a "menu cooldown" so the menus stay open or closed instead of flashing
                                        let _  = hidden_propagates.insert(lower_panel, HiddenPropagate::new()).unwrap(); 
                                        self.focused = true; // TODO: this .focused can be replaced with just a .is_some()
                                        self.focused_ent = Some(ent);

                                    } else  {
                                        let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                                        let _  = hidden_propagates.remove(lower_panel).unwrap();
                                        self.focused = false;
                                        self.focused_ent = None; 

                                    }
                                }
                            }
                        }
                    }
                } else if input.mouse_button_is_down(MouseButton::Left) && self.focused{ // if
                    // focused and click anywhere on the game map, unfocus all
                    // this code checks that the tile clicked is not that of the
                    // focused entity, if we didnt click on the focused entity
                    // then exit focus mode. This is a pretty important UX
                    // function as it stops the ui elements from flashing.
                    let mut cfocus = false;
                    for (building, build_tile_pos) in (&buildings, &tileposs).join(){
                        if &mouse_tile_pos.pos == build_tile_pos{
                            cfocus = true;
                            break
                        }
                    }
                    if let (Some(interact_menu), Some(lower_panel)) = (ui_finder.find("barracks_menu"), ui_finder.find("lower_panel")){
                        if !cfocus{
                            let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                            let _  = hidden_propagates.remove(lower_panel).unwrap();
                            self.focused = false;
                        }
                    }
                }
            }


        } else {
            // Checks if output location selector is in range of tile, if not hide it
            if (mouse_tile_pos.pos.x >= tileposs.get(self.focused_ent.unwrap()).unwrap().x -2 && mouse_tile_pos.pos.x <= tileposs.get(self.focused_ent.unwrap()).unwrap().x + 2) && (mouse_tile_pos.pos.y >= tileposs.get(self.focused_ent.unwrap()).unwrap().y -2 && mouse_tile_pos.pos.y <= tileposs.get(self.focused_ent.unwrap()).unwrap().y + 2){
                for fol in (&mut follower).join(){
                    fol.kind = TileType::Location; // TODO show output location when focused?
                };
                if input.mouse_button_is_down(MouseButton::Left) && self.focused {
                    outposs.get_mut(self.focused_ent.unwrap()).unwrap().pos = mouse_tile_pos.pos.clone();
                    self.location_mode = false;
                }
            } else {
                for fol in (&mut follower).join(){ fol.kind = TileType::Empty  }
            }
        }

        // This iterator detects whenever some of the buttons within the
        // interact menu are clicked, if so perform the appropriate action
        for event in events.read(&mut self.event_reader){
            if event.event_type == UiEventType::Click{
                let clicked = event.target.id();
                if clicked == ui_finder.find("Warrior_button").unwrap().id(){
                    unitstacks.get_mut(self.focused_ent.unwrap()).unwrap().push(TileType::Warrior); // Push a tile of type warrior to the focused building's unit stack
                    // Done: decide on health values for units!
                    // TODO add some visual feedback for when a unit is pushed to the stack. Im thinking some text above the build menu that fades after 10s? - Not in scope
                }else if clicked == ui_finder.find("Heavy_button").unwrap().id(){
                    unitstacks.get_mut(self.focused_ent.unwrap()).unwrap().push(TileType::Heavy); // Push a tile of type heavy to the focused building's unit stack
                }else if clicked == ui_finder.find("Monk_button").unwrap().id(){
                    unitstacks.get_mut(self.focused_ent.unwrap()).unwrap().push(TileType::Monk); // Push a tile of type monk to the focused building's unit stack
                } else if clicked == ui_finder.find("Location_button").unwrap().id(){
                    self.location_mode = !self.location_mode; // Toggle location mode
                    for fol in (&mut follower).join(){ fol.kind = Empty };
                } else if clicked == ui_finder.find("Cancel_button").unwrap().id(){ // TODO Cancel isnt very descriptive, try replacing with remove unit / pop unit?
                    unitstacks.get_mut(self.focused_ent.unwrap()).unwrap().pop(); // pop a unit from the stack
                }
            }
        }
    }
}
