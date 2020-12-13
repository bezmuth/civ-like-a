use crate::game::{Build, Building, TileType, MouseTilePos, OnUi, PlayersInfo, TilePos, Follower, UnitStack, Unit, OutPos, Stat, Tiles, Layer3, Layer2, Turn};
use TileType::Empty;
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::ecs::Storage;
use amethyst::ecs::storage;

use amethyst::{core::HiddenPropagate, ui::{UiEventType, UiText}, ecs::WriteStorage, ecs::prelude::{System, ReadStorage, ReadExpect, SystemData, Join, WriteExpect}, input::{InputHandler, StringBindings}, shred::Read, shred::World, shred::Write, shrev::EventChannel, shrev::ReaderId, ui::UiEvent, ui::UiFinder, winit::MouseButton, renderer::{SpriteRender}};

extern crate rand;

use rand::thread_rng;
use rand::Rng;

pub struct UnitInteractSystem{
    event_reader: ReaderId<UiEvent>,
    focused_ent: Option<Entity>,
    focused: bool,
    valid_move: bool,
    last_turn: i32,
    turns_til_claim: Option<i32>,
}

impl UnitInteractSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let event_reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader(); // Do this whenever trying to read events
        Self {  event_reader, focused_ent : None, focused : false, valid_move : true, last_turn : 0, turns_til_claim: None}
    }
}

impl<'s> System<'s> for UnitInteractSystem {
    type SystemData = (
        WriteStorage<'s, TilePos>,
        ReadExpect<'s, MouseTilePos>,
        UiFinder<'s>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, PlayersInfo>,
        WriteStorage<'s, Unit>,
        WriteStorage<'s, Stat>,
        Write<'s, EventChannel<UiEvent>>,
        ReadExpect<'s, OnUi>,
        Entities<'s>,
        ReadExpect<'s, Build>,
        WriteStorage<'s, Follower>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Layer3>,
        WriteStorage<'s, Tiles>,
        ReadExpect<'s, Turn>,
        WriteStorage<'s, Layer2>,
        WriteStorage<'s, Building>
    );

    fn run(&mut self, (mut tileposs, mouse_tile_pos, ui_finder, input, playersinfo,mut units, mut stats, events, onui,mut entities, build, mut follower, mut spriterenderers, mut layer3, mut tiles, turn, mut layer2, buildings): Self::SystemData) {
        if !onui.case{ // first check that the ui is not being interacted with


            // when we leave location mode, return the follower tile to that of
            // an empty sprite This is kinda haky but I didnt see the point of
            // adding a "just focused" variable, instead we check if the focused
            // ent is set to something, if thats the case then we must have just
            // left focused mode.
            for fol in (&mut follower).join(){
                if fol.kind == TileType::Location && self.focused_ent.is_some() && !self.focused{
                    self.focused_ent = None;
                    fol.kind = TileType::Empty
                }
            }


            // Pretty much this entire code block just changes the mode to focused and adds the clicked unit to focused_ent
            if input.mouse_button_is_down(MouseButton::Left) && (build.mode.is_none() || build.mode.unwrap() != TileType::Demolish) && !self.focused{  // dont interact with units when in move mode
                for (unit, unit_tile_pos, ent) in (&units, &tileposs, &entities).join(){
                    if (& mouse_tile_pos.pos == unit_tile_pos) 
                        && unit.playernum == playersinfo.current_player_num
                    {
                        self.focused = true;
                        self.focused_ent = Some(ent)
                    }
                }
            } else if input.mouse_button_is_down(MouseButton::Left) && self.focused {
                // all this code checks that the tile clicked is not that of the
                // focused entity, if we didnt click on the focused entity then
                // exit focus mode
                let mut cfocus = false;
                for (_, unit_tile_pos) in (&units, &tileposs).join(){
                    if &mouse_tile_pos.pos == unit_tile_pos{
                        cfocus = true;
                        break
                    }
                }
                if !cfocus && !self.valid_move{
                    self.focused = false;
                }
            }
            if turn.num > self.last_turn{ // ensures we cant interact with other player's units and ensures a unit is not selected when turn starts
                self.focused = false;
                self.focused_ent = None;
                self.last_turn = turn.num;
                if let Some(mut ttc) = self.turns_til_claim{
                    self.turns_til_claim = Some(ttc - 1);
                }
            }else {
                if self.focused{
                    let focused_pos = tileposs.get(self.focused_ent.unwrap()).unwrap().clone();
                    let focused_unit = units.get(self.focused_ent.unwrap()).unwrap();
                    let mut combat_mode = false;

                    // checks if a enemy unit is within attacking range, if so enter
                    // combat mode blocking movement and forcing an attack
                    for (unit, pos) in (&units, &tileposs).join(){
                        if unit.playernum != playersinfo.current_player_num {
                            if ((pos.x - focused_pos.x).pow(2) as f32 + (pos.y - focused_pos.y).pow(2) as f32).sqrt() <= 1.0{
                                combat_mode = true;
                            }
                        }
                    }
                    // WARNING: HERE BE DRAGONS. Most of this code is just fiddling
                    // with the borrow checker till it does what I want it to. This
                    // code is split into two sections, one for combat mode and one
                    // for non combat mode. In combat mode: one must attack a nearby
                    // unit. In non combat mode (the else block) the unit can move
                    // freely.
                    if combat_mode{
                        let mut target_ent : Option<Entity> = None;
                        if input.mouse_button_is_down(MouseButton::Left) && !stats.get(self.focused_ent.unwrap()).unwrap().exhausted{ // direct refrences using the stats.get so the borrow checker doesnt panick
                            for (unit, pos, ent) in (&units, &tileposs, &entities).join(){ // First we get the entity of the unit the player is trying to attack
                                if pos == &mouse_tile_pos.pos && unit.playernum != playersinfo.current_player_num {
                                    target_ent = Some(ent);
                                }
                            }
                            if let Some(mut t_ent) = target_ent{ // check if the player has attempted an attack
                                let mut t_stat = *stats.get(t_ent).unwrap(); // set the t_stat var to a "copy" of the target units stats
                                let focused_stats = *stats.get_mut(self.focused_ent.unwrap()).unwrap(); // do the same for the focused unit
                                // Run the calculations to see how much health the target unit will loose
                                let mut rng = thread_rng();
                                if rng.gen_range(0., 1.) >= t_stat.resistance{
                                    if rng.gen_range(0.,1.) <= focused_stats.crit_chance{
                                        t_stat.health -= focused_stats.attack * 1.5
                                    } else {
                                        t_stat.health -= focused_stats.attack;
                                    }
                                    for _ in 0..(focused_stats.multi_hit_amount as i32){
                                        if rng.gen_range(0., 1.) <= t_stat.multi_hit_chance{
                                            t_stat.health -= focused_stats.attack;
                                        }
                                    }
                                }
                                println!("OTHER: {}| SELF: {}", t_stat.health, focused_stats.health);
                                // Actually set the target units health from the
                                // copy of the target unit's health
                                stats.get_mut(t_ent).unwrap().health = t_stat.health;
                                stats.get_mut(self.focused_ent.unwrap()).unwrap().exhausted = true;

                                if t_stat.health <= 0. { // if the unit is dead
                                    entities.delete(t_ent).expect("Couldnt delete unit?"); // remove it from the world
                                    let t_pos = tileposs.get(t_ent);
                                    // and then set the spriterender to nothing
                                    for (tile, spriterender, pos, _) in (& tiles, &mut spriterenderers,  & tileposs, & layer3).join() {
                                        if pos == t_pos.unwrap(){
                                            spriterender.sprite_number = TileType::Empty as usize; // blank sprite;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        let focused_stats = stats.get_mut(self.focused_ent.unwrap()).unwrap();
                        // checks if the mouse is in range of the unit's speed and that the unit is not exhausted and that the attempted move is not onto the same tile.
                        self.valid_move = ((mouse_tile_pos.pos.x - focused_pos.x).pow(2) as f32 + (mouse_tile_pos.pos.y - focused_pos.y).pow(2) as f32).sqrt() <= focused_stats.speed && !focused_stats.exhausted && mouse_tile_pos.pos != focused_pos;
                        for fol in (&mut follower).join(){
                            if self.valid_move{
                                fol.kind = TileType::Location
                            } else {
                                fol.kind = TileType::Empty
                            }
                        }
                        // theres a lot of wrestling with the borrow checker in this code block.
                        // Kinda makes me feel that something might be wrong.
                        if self.valid_move && input.mouse_button_is_down(MouseButton::Left) {
                            focused_stats.exhausted = true; // ensures this is the only move the unit makes this turn
                            let mut future_pos : Option<TilePos> = None;
                            for (spriterender, pos, _, _) in (&mut spriterenderers, &mut tileposs, &mut layer3, &mut tiles).join() {
                                if & mouse_tile_pos.pos == pos{
                                    spriterender.sprite_number = focused_unit.unit_type as usize;
                                    future_pos = Some(*pos);
                                } else if focused_pos == *pos {
                                    spriterender.sprite_number = TileType::Empty as usize;
                                }
                            }
                            if let Some(fut_pos) = future_pos{
                                let focused_pos = tileposs.get_mut(self.focused_ent.unwrap()).unwrap();
                                *focused_pos = fut_pos;
                                self.focused = false;
                            }
                        }
                        for (tile, spriterender, pos, _) in (&mut tiles, &mut spriterenderers,  &mut tileposs, &mut layer2).join() {// if the target tile is a city centre of other player, destroy it.
                            if focused_pos == *pos{
                                for (building, ent) in (& buildings, &*entities).join(){
                                    if tile.tile_type.is_some() && building.playernum != playersinfo.current_player_num {
                                        if let Some(ttc) = self.turns_til_claim{
                                            println!("{}", ttc);
                                            if ttc == 0{
                                                entities.delete(ent).expect("Could not delete this building, does it exist?");
                                                spriterender.sprite_number = TileType::Empty as usize; // blank sprite;
                                                tile.tile_type = None;
                                                self.turns_til_claim = None;
                                                break;
                                            }
                                        } else {
                                            self.turns_til_claim = Some(2); // if the number of players is 2, this would need to change. Out of scope
                                        }
                                    }
                                }

                            }
                        }

                    }
                }
            }
        }
    }
}
