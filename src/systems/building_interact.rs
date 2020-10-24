use crate::game::{Building, BuildingType, Layer2, MouseTilePos, PlayersInfo, Tiles};
use amethyst::{core::{Transform, HiddenPropagate}, derive::SystemDesc, ui::UiEventType, ecs::WriteStorage, ecs::prelude::{System, ReadStorage, ReadExpect, SystemData, Join, WriteExpect}, input::{InputHandler, StringBindings}, shred::Read, shred::World, shred::Write, shrev::EventChannel, shrev::ReaderId, ui::UiEvent, ui::UiFinder, ui::UiTransform, winit::MouseButton};



pub struct BuildingInteractSystem{
    event_reader: ReaderId<UiEvent>,
    first_run: bool,
}

impl BuildingInteractSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let event_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader();
        Self {  event_reader, first_run:true } // * gotta do this whenever trying to read events
    }
}

impl<'s> System<'s> for BuildingInteractSystem {
    type SystemData = (
        ReadStorage<'s, Tiles>,
        ReadStorage<'s, Layer2>,
        ReadExpect<'s, MouseTilePos>,
        UiFinder<'s>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, HiddenPropagate>, // * hides widget and all children
        ReadExpect<'s, PlayersInfo>,
        ReadStorage<'s, Building>,
        Write<'s, EventChannel<UiEvent>>,
        ); 
        // * quick note on wanting to do a small menu above the tile, the location of a Ui element (a UiTransform) cannot be written to
        // * therefore I either have to create a static menu, for instance a menu that replaces the lower panel like aoe2
        // * OR I could move the camera so the menu apears above the tile
    fn run(&mut self, (tiles, layer2, mouse_tile_pos, ui_finder, input, mut hidden_propagates, playersinfo, buildings, events): Self::SystemData) {        
        if self.first_run{ // * runs on first execute to ensure the barracks menu is hidden
            if let Some(interact_menu) = ui_finder.find("barracks_menu"){
                let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                self.first_run = false;
            }
        } 
        if input.mouse_button_is_down(MouseButton::Left){ 
            for (tile, _) in (&tiles, &layer2).join(){
                for building in buildings.join(){
                    if  (mouse_tile_pos.x == tile.x) && (mouse_tile_pos.y == tile.y) //todo: the "this tile has been clicked" should really be its own function/event
                    && building.x == mouse_tile_pos.x
                    && building.y == mouse_tile_pos.y
                    && building.buildingtype == BuildingType::Barrack
                    && building.playernum == playersinfo.current_player_num 
                    {
                        if let (Some(interact_menu), Some(lower_panel)) = (ui_finder.find("barracks_menu"), ui_finder.find("lower_panel")){
                            if hidden_propagates.contains(interact_menu){
                                let _  = hidden_propagates.remove(interact_menu).unwrap(); // todo: implement a "menu cooldown" so the menus stay open or closed instead of flashing
                                let _  = hidden_propagates.insert(lower_panel, HiddenPropagate::new()).unwrap(); 
                                
                            } else  {
                                let _  = hidden_propagates.insert(interact_menu, HiddenPropagate::new()).unwrap();
                                let _  = hidden_propagates.remove(lower_panel).unwrap();
                                
                            }
                        }
                    }
                }
            }
        }

        for event in events.read(&mut self.event_reader){
            if event.event_type == UiEventType::Click{
                let clicked = event.target.id();
                if clicked == ui_finder.find("Warrior_button").unwrap().id(){

                } else if clicked == ui_finder.find("Location_button").unwrap().id(){
                    
                }
            }
        }

    }
    
}