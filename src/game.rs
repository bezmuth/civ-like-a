use std::time::Duration;
use rand::prelude::*;
use amethyst::{assets::{AssetStorage, Handle, Loader}, core::{ArcThreadPool, frame_limiter::{FrameLimiter, FrameRateLimitStrategy}, transform::Transform}, ecs::{Component, DenseVecStorage}, input::{is_key_down, VirtualKeyCode}, prelude::*, renderer::{
    Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,  resources::Tint, palette::Srgba,
    }, shred::{DispatcherBuilder}, shred::Dispatcher, ui::{
        Anchor, UiCreator, UiText, UiTransform, TtfFormat, LineMode,
    }};

use super::systems;
pub use super::components::{
    Tiles,
    TilePos,
    Player,
    PlayersInfo,
    Build,
    Building,
    TileType,
    Resbar,
    Layer1,
    Layer2,
    Layer3,
    UnitStack,
    Unit,
    OutPos,
    Stat,

};

use crate::win::Win;



#[derive(Default)]
pub struct Civ<'a, 'b> {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for Civ<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Create the DispatcherBuilder and register some Systems that should only run for this State.
        let mut dispatcher_builder = DispatcherBuilder::new();
        //todo: reorder these
        dispatcher_builder.add(systems::Imgui{toggled: false}, "imgui", &[]);
        dispatcher_builder.add(systems::M2TileSystem, "m2tile_system", &[]);
        dispatcher_builder.add(systems::UIDetect, "uidetect_system", &[]);
        dispatcher_builder.add(systems::SheetSystem, "sheet_system", &["m2tile_system"]);
        dispatcher_builder.add(systems::CameraSystem{multiplier:1.}, "camera_system", &["sheet_system"]);
        dispatcher_builder.add(systems::BuildSystem::default(), "build_system", &["sheet_system", "camera_system"]); // https://doc.rust-lang.org/std/default/trait.Default.html
        dispatcher_builder.add(systems::TurnSystem::new(world), "turn_system", &["build_system"]); // * turn system depends on build system because build system inits the UiEvent reader into the world, turn system reads it
        dispatcher_builder.add(systems::ResourceCalcSystem{last_turn : -1}, "resourcecalc_system", &["sheet_system", "camera_system", "build_system", "turn_system"]);
        dispatcher_builder.add(systems::ResourceDispSystem, "resourcedisp_system", &["sheet_system", "camera_system", "build_system", "resourcecalc_system", "turn_system"]);
        dispatcher_builder.add(systems::TerrainGenSystem::new(), "terraingen_system", &["sheet_system", "camera_system", "build_system", "resourcecalc_system", "turn_system"]);
        dispatcher_builder.add(systems::BuildingInteractSystem::new(world), "building_interact_system", &["sheet_system", "camera_system", "build_system", "resourcecalc_system", "turn_system"]);
        dispatcher_builder.add(systems::TileMouseFollow, "tile_mouse_follow_system", &["sheet_system", "build_system", "building_interact_system"]);
        dispatcher_builder.add(systems::UnitTurnSystem{last_turn : -1}, "unit_turn_system", &["sheet_system", "build_system", "building_interact_system", "turn_system", "resourcecalc_system"]);
        dispatcher_builder.add(systems::UnitInteractSystem::new(world), "unit_interact_system", &["sheet_system", "build_system", "building_interact_system", "turn_system", "resourcecalc_system", "unit_turn_system"]);
        dispatcher_builder.add(systems::WinCheckSystem, "win_check_system", &["sheet_system", "build_system"]);

        // Build and setup the Dispatcher.
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
        // Load the spritesheet necessary to render the graphics.
        // spritesheet is the layout of the sprites on the image;
        // texture is the pixel data.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));


        // * limits fps to 60
        world.insert(FrameLimiter::new(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(14)), 60)); // 16.8 ms in a frame, 14 ms for sleep about 2 ms for yeild


        world.register::<Layer1>();
        world.register::<Layer2>();
        world.register::<Layer3>();

        initialise_camera(world);
        // TODO: move all these to their own init?

        // RESOURCE INIT
        world.insert(Build {mode: None}); // * WORLD.INSERT WORKS WITH
        // RESOURCES, RESOURCES are like components except they only hold a single
        // struct as opposed to a list of structs, they are accessed in a similar way
        world.insert(MouseTilePos{ pos : TilePos{x:0 , y:0} });
        world.insert(OnUi{case: false});
        world.insert(Turn{num:0});
        world.insert(WinState{case:false, num_winner : 0});


        // This inits the players
        let playercount = 2;
        world.insert(PlayersInfo{count: playercount, current_player_num:0});
        for x in 0..(playercount){
            world // * WORLD.INSERT DOES NOT WORK WITH COMPONENTS
                .create_entity()
                .with(Player::new(x))
                .build();
        }

        // COMPONENT INIT
        initialise_follow_ent(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_unit_sheet(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_building_sheet(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_world_sheet(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_res_disp(world);
        initialise_lower_menu(world);
        initialise_interact_menus(world);

    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape){
                // Go back to the Menu state when escape pressed.
                data.world.delete_all();
                return Trans::Pop;
            }
        }
        // Escape isn't pressed, so we stay in this State. The function requires
        // a return so we return the "None" type for the state transition enum
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        // updates systems when update occurs
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        let winstate = data.world.read_resource::<WinState>();
        if winstate.case{
            println!("WINSATE PUSHED");
            return Trans::Push(Box::new(Win{winner : winstate.num_winner}));
        } else {
            Trans::None
        }
    }
}

//todo: move all these into a "component" file?
pub struct Turn{
    pub num: i32,
}
#[derive(PartialEq, Eq)] // Partial Eq and Eq required for comparisons with TilePos
pub struct MouseTilePos{
    pub pos: TilePos,
}

pub struct OnUi{ // if the mouse is on a Ui Element this will be true
    pub case: bool,
}

// contains data that describes what type of tile the follower is. This
// couldnt realy be grouped with other components in the component folder, so
// here it shall remain
pub struct Follower{
    pub kind : TileType,
}
impl Component for Follower {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct WinState{
    pub case : bool,
    pub num_winner : i32,

}


fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // sprite_sheet is the layout of the sprites on the image
    // texture_handle is a cloneable reference to the texture

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // This loads the file describing where the individual sprites in the sprite
    // sheet are located.
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(1280./2.0, 720./2.0, 1.0); //TODO: increase unscaled res? - more tiles on screen?

    world
        .create_entity()
        .with(Camera::standard_2d(1280., 720.)) // * width is halved in spritesheet.ron
        .with(transform)
        .build();
}

// This function inits the lowest layer of the tilesheet, creating a layer of
// tiles which only contain grass tiles for now, until the terrain gen system
// replaces them with the appropriate titles.
fn initialise_world_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // This lambda just makes it easier to understand what is going on later in
    // the code, it converts a tile map position to a screen/world position. It
    // is pretty important and underpins most of my isometric graphics. The
    // inverse of it is used in the mouse2tile system.
    let tile_to_screen = |x,y| (((x-y) as f32 * 32.), ((x+y) as f32 * 17.)); // 0 is x pos, 1 is y pos

    let mut transform = Transform::default();
    let mut sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: TileType::Grass as usize,
    };

    for x in 0..50{
        for y in 0..50{
            transform.set_translation_xyz(tile_to_screen(x,y).0, tile_to_screen(x,y).1, 0.0);
            // screen.x = (map.x - map.y) * TILE_WIDTH_HALF;
            // screen.y = (map.x + map.y) * TILE_HEIGHT_HALF;

            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() > 0.99 { // this randomly generates the ruins that the centers are built upon
                sprite_render.sprite_number = TileType::Ruins as usize;
                world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { player: 0, tile_type: Some(TileType::Ruins)})
                .with(TilePos{x, y})
                .with(Layer1)
                .build();
            } else{
                sprite_render.sprite_number = TileType::Grass as usize;
                world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { player: 0, tile_type: Some(TileType::Grass)})
                .with(TilePos{x, y})
                .with(Layer1)
                .build();
            }
        }
    }
}

// This initalises the layer above the terrain sheet which displayes player
// created buildings
fn initialise_building_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let tile_to_screen = |x,y| (((x-y) as f32 * 32.), ((x+y) as f32 * 17.)); // 0 is x pos, 1 is y pos

    let mut transform = Transform::default();

    let tint = Tint(Srgba::new(1.0, 1., 1., 1.));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 4 // blank sprite
    };
    for x in 0..50{
        for y in 0..50{
            transform.set_translation_xyz(tile_to_screen(x,y).0, tile_to_screen(x,y).1, 0.00001); // z > 0 so it is displayed above layer 0
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { player: 0, tile_type: None})
                .with(TilePos{x, y})
                .with(Layer2)
                .with(tint)
                .build();
        }
    }
}

// This is the third layer, it contains units.
fn initialise_unit_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let tile_to_screen = |x,y| (((x-y) as f32 * 32.), ((x+y) as f32 * 17.)); // 0 is x pos, 1 is y pos
    let tint = Tint(Srgba::new(1.0, 1., 1., 1.));
    let mut transform = Transform::default();
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 4 // blank sprite
    };
    for x in 0..50{
        for y in 0..50{
            transform.set_translation_xyz(tile_to_screen(x,y).0, tile_to_screen(x,y).1, 0.00001); // z > 0 so it is displayed above layer 0
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { player: 0, tile_type: None})
                .with(TilePos{x, y})
                .with(Layer3)
                .with(tint)
                .build();
        }
    }
}



fn initialise_res_disp(world: &mut World){
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let top_info = UiTransform::new(
        "resbar_top".to_string(),
        Anchor::TopMiddle, 
        Anchor::Middle,
        0., 
        -30., 
        0., 
        1920., 
        25.,
    );

    let top = world
        .create_entity()
        .with(top_info)
        .with(UiText::new(
            font.clone(),
            format!("RESBAR!"),
            [1., 1., 1., 1.],
            25.,
            LineMode::Single,
            Anchor::TopMiddle,
        ))
        .build();


    world.insert(Resbar {top});
}

// Dynamically loads the lower menu from a .ron file. These are called prefabs
// in amethyst
fn initialise_lower_menu(world: &mut World){
    world.exec(|mut creator: UiCreator<'_>| {
        creator.create("ui/lower_panel.ron", ());
        creator.create("ui/build_menu.ron", ());
    });
}

// Another prefab load, this one handles when the user clicks on a barrack
fn initialise_interact_menus(world: &mut World){
    world.exec(|mut creator: UiCreator<'_>| {
        creator.create("ui/barracks_menu.ron", ());
    });
}
// inits the entity that will follow the mouse when tile_mouse_follow is running
fn initialise_follow_ent(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let transform = Transform::default();
    // This entity is not part of the sheet, thats so I dont have to constantly
    // r&w to the sheet. Removes the need to store what was on the tile and
    // replace it when the mouse moves. This way I only move a sprite to the
    // translation of a tile instead of on the sheet. If it had been part of the
    // sheet it would have probably gone on layer3 (the unit layer)
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 4 as usize,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(transform.clone())
        .with(Follower{kind: TileType::Empty})
        .build();


}
