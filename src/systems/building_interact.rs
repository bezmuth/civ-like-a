use crate::game::{Build, Building, TileType, MouseTilePos, OnUi, PlayersInfo, TilePos, Follower, UnitStack, Unit};
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::ecs::Storage;
use amethyst::ecs::storage;

use amethyst::{core::HiddenPropagate, ui::{UiEventType, UiText}, ecs::WriteStorage, ecs::prelude::{System, ReadStorage, ReadExpect, SystemData, Join, WriteExpect}, input::{InputHandler, StringBindings}, shred::Read, shred::World, shred::Write, shrev::EventChannel, shrev::ReaderId, ui::UiEvent, ui::UiFinder, winit::MouseButton};

fn StackUiUpdate(stack : &mut UnitStack, ui_finder : UiFinder, mut ui_text : Storage<'_, UiText, amethyst::shred::FetchMut<'_, storage::MaskedStorage<UiText>>, >){ // * this function updates the stack ui elements, its a bit a gross function but I cannot iterate over Ui elements from arbritary indexes
    if let Some(peeked) = stack.peek(){
        let mut button_text = "undefined";
        match peeked.UnitType{
            TileType::Warrior => button_text = "W",
            _ => button_text = "undefined",
        }
        // This is ugly but self explanitory
        match stack.top{
            7 => ui_text.get_mut(ui_finder.find("Stack_Button8").unwrap()).unwrap().text = button_text.to_string(),
            6 => ui_text.get_mut(ui_finder.find("Stack_Button7").unwrap()).unwrap().text = button_text.to_string(),
            5 => ui_text.get_mut(ui_finder.find("Stack_Button6").unwrap()).unwrap().text = button_text.to_string(),
            4 => ui_text.get_mut(ui_finder.find("Stack_Button5").unwrap()).unwrap().text = button_text.to_string(),
            3 => ui_text.get_mut(ui_finder.find("Stack_Button4").unwrap()).unwrap().text = button_text.to_string(),
            2 => ui_text.get_mut(ui_finder.find("Stack_Button3").unwrap()).unwrap().text = button_text.to_string(),
            1 => ui_text.get_mut(ui_finder.find("Stack_Button2").unwrap()).unwrap().text = button_text.to_string(),
            0 => ui_text.get_mut(ui_finder.find("Stack_Button1").unwrap()).unwrap().text = button_text.to_string(),
            _ => panic!("What, how?")
        }

    } 

}

pub struct BuildingInteractSystem{
    event_reader: ReaderId<UiEvent>,
    first_run: bool,
    focused: bool,
    location_mode: bool,
    focused_ent: Option<Entity>,
}

impl BuildingInteractSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let event_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader(); // * gotta do this whenever trying to read events
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
        WriteStorage<'s, UiText>,
        ); 
        // * quick note on wanting to do a small menu above the tile, the location of a Ui element (a UiTransform) cannot be written to
        // * therefore I either have to create a static menu, for instance a menu that replaces the lower panel like aoe2
        // * OR I could move the camera so the menu apears above the tile
    fn run(&mut self, (tileposs, mouse_tile_pos, ui_finder, input, mut hidden_propagates, playersinfo, buildings, events, build, onui, mut follower, mut unitstacks, entities, mut ui_text): Self::SystemData) {        
        if !self.location_mode{ // if operating normally
            for fol in (&mut follower).join(){ fol.kind = TileType::Empty }

            if self.first_run{ // * runs on first execute to ensure the barracks menu is hidden
                if let Some(interact_menu) = ui_finder.find("barracks_menu"){
                    let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                    self.first_run = false;
                }
            }

            if !onui.case{ // first check that the ui is not being interacted with
                if input.mouse_button_is_down(MouseButton::Left) && (build.mode.is_none() || build.mode.unwrap() != TileType::Demolish) && !self.focused{  // dont interact with buildings when in build mode
                    for (building, building_tile_pos, ent) in (&buildings, &tileposs, &entities).join(){
                        if (& mouse_tile_pos.pos == building_tile_pos) //todo: the "this tile has been clicked" should really be its own function/event
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
                } else if input.mouse_button_is_down(MouseButton::Left) && self.focused{ // if focused and click anywhere on the game map, unfocus
                    if let (Some(interact_menu), Some(lower_panel)) = (ui_finder.find("barracks_menu"), ui_finder.find("lower_panel")){
                        let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                        let _  = hidden_propagates.remove(lower_panel).unwrap();
                        self.focused = false;
                    }
                }
            }


        } else {
            for fol in (&mut follower).join(){ fol.kind = TileType::Location }
        }
        
        let mut focusedstack : Option<&mut UnitStack> = None;
        
        for event in events.read(&mut self.event_reader){
            if event.event_type == UiEventType::Click{
                let clicked = event.target.id();
                if clicked == ui_finder.find("Warrior_button").unwrap().id(){
                    unitstacks.get_mut(self.focused_ent.unwrap()).unwrap().push(Unit{UnitType: TileType::Warrior, Health: 0}); // TODO: decide on health values for units!
                    focusedstack = unitstacks.get_mut(self.focused_ent.unwrap());

                } else if clicked == ui_finder.find("Location_button").unwrap().id(){
                    self.location_mode = !self.location_mode;
                } else if clicked == ui_finder.find("Stack_Button1").unwrap().id()
                    || clicked == ui_finder.find("Stack_Button2").unwrap().id()
                    || clicked == ui_finder.find("Stack_Button3").unwrap().id()
                    || clicked == ui_finder.find("Stack_Button4").unwrap().id()
                    || clicked == ui_finder.find("Stack_Button5").unwrap().id()
                    || clicked == ui_finder.find("Stack_Button6").unwrap().id()
                    || clicked == ui_finder.find("Stack_Button7").unwrap().id()
                    || clicked == ui_finder.find("Stack_Button8").unwrap().id() { // * As UI elements cannot be layered on top of each other I cannot make one big "stack" button, instead I have to address each stack button in this if statement

                        let test = unitstacks.get_mut(self.focused_ent.unwrap()).unwrap().pop().unwrap().UnitType;
                        println!("Popped: {}", test as i32);

                        focusedstack = unitstacks.get_mut(self.focused_ent.unwrap());


                }
            }
        }

        if let Some(focusedstack_unwrap) = focusedstack{ // had to move here because of borrow checker
            StackUiUpdate(focusedstack_unwrap, ui_finder, ui_text);
        }

    }
}
