use crate::game::{Build, Building, TileType, Layer2, MouseTilePos, OnUi, PlayersInfo, TilePos, Tiles, Follower};
use amethyst::{core::{Transform, HiddenPropagate}, derive::SystemDesc, ui::UiEventType, ecs::WriteStorage, ecs::prelude::{System, ReadStorage, ReadExpect, SystemData, Join, WriteExpect}, input::{InputHandler, StringBindings}, shred::Read, shred::World, shred::Write, shrev::EventChannel, shrev::ReaderId, ui::UiEvent, ui::UiFinder, ui::UiTransform, winit::MouseButton};



pub struct BuildingInteractSystem{
    event_reader: ReaderId<UiEvent>,
    first_run: bool,
    focused: bool,
    location_mode: bool,
}

impl BuildingInteractSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let event_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader(); // * gotta do this whenever trying to read events
        Self {  event_reader, first_run:true , focused: false, location_mode: false } 
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
        ); 
        // * quick note on wanting to do a small menu above the tile, the location of a Ui element (a UiTransform) cannot be written to
        // * therefore I either have to create a static menu, for instance a menu that replaces the lower panel like aoe2
        // * OR I could move the camera so the menu apears above the tile
    fn run(&mut self, (tileposs, mouse_tile_pos, ui_finder, input, mut hidden_propagates, playersinfo, buildings, events, build, onui, mut follower): Self::SystemData) {        
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
                    for (building, building_tile_pos) in (&buildings, &tileposs).join(){
                        if (& mouse_tile_pos.pos == building_tile_pos) //todo: the "this tile has been clicked" should really be its own function/event
                        && building.playernum == playersinfo.current_player_num 
                        {
                            if building.TileType == TileType::Barrack{ // if clicking on barrack
                                if let (Some(interact_menu), Some(lower_panel)) = (ui_finder.find("barracks_menu"), ui_finder.find("lower_panel")){
                                    if hidden_propagates.contains(interact_menu){
                                        let _  = hidden_propagates.remove(interact_menu).unwrap(); // todo: implement a "menu cooldown" so the menus stay open or closed instead of flashing
                                        let _  = hidden_propagates.insert(lower_panel, HiddenPropagate::new()).unwrap(); 
                                        self.focused = true;
                                        
                                    } else  {
                                        let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                                        let _  = hidden_propagates.remove(lower_panel).unwrap();
                                        self.focused = false;
                                        
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

        for event in events.read(&mut self.event_reader){
            if event.event_type == UiEventType::Click{
                let clicked = event.target.id();
                if clicked == ui_finder.find("Warrior_button").unwrap().id(){

                } else if clicked == ui_finder.find("Location_button").unwrap().id(){
                    self.location_mode = !self.location_mode;
                }
            }
        }
        

    }
    
}