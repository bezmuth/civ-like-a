use std::time::Duration;
use amethyst::{assets::{AssetStorage, Handle, Loader}, core::{ArcThreadPool, HiddenPropagate, frame_limiter::{FrameLimiter, FrameRateLimitStrategy}, transform::Transform}, ecs::{Component, DenseVecStorage, NullStorage}, input::{is_key_down, VirtualKeyCode}, prelude::*, renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
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
};

#[derive(Default)]
pub struct Civ<'a, 'b> {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}
// TODO: !! implement turn system !!

impl<'a, 'b> SimpleState for Civ<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Create the `DispatcherBuilder` and register some `System`s that should only run for this `State`.
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


        // Build and setup the `Dispatcher`.
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));


        // * limits fps to 60
        world.insert(FrameLimiter::new(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(14)), 60)); // 16.8 ms in a frame, 14 ms for sleep about 2 ms for yeild


        world.register::<Layer1>();
        world.register::<Layer2>();
        world.register::<Layer3>();

        initialise_camera(world);
        // TODO: move all these to their own init?
        world.insert(Build {mode: None}); // * WORLD.INSERT WORKS WITH RESOURCES
        // world.insert(CurrentPlayer{ playernum: 0}); 
        world.insert(MouseTilePos{ pos : TilePos{x:0 , y:0} });
        world.insert(OnUi{case: false});
        world.insert(Turn{num:0});


        let playercount = 2; // todo: move this and the player gen logic to a system?
        world.insert(PlayersInfo{count: playercount, current_player_num:0});
        for x in 0..(playercount){
            world // * WORLD.INSERT DOES NOT WORK WITH COMPONENTS
                .create_entity()
                .with(Player::new(x))                                   
                .build();
        } 

        initialise_follow_ent(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_overlay_sheet(world, self.sprite_sheet_handle.clone().unwrap());    
        initialise_world_sheet(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_res_disp(world);
        initialise_lower_menu(world);
        initialise_interact_menus(world);

    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape){
                // Go back to the Menu state.
                data.world.delete_all();
                return Trans::Pop;
            }
        }
        // Escape isn't pressed, so we stay in this State.
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // updates systems when update occurs
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}

//todo: move all these into a "component" file?
pub struct Turn{
    pub num: i32,
}
#[derive(PartialEq, Eq)] // Partial Eq and Eq for comparisons with TilePos
pub struct MouseTilePos{
    pub pos: TilePos,
}

pub struct OnUi{ // if the mouse is on a Ui Element this will be true
    pub case: bool,
}

pub struct Follower{
    pub kind : TileType,
} // contains data that describes what type of tile the follower is // ! this is a component that is not in a component file
impl Component for Follower {
    type Storage = DenseVecStorage<Self>;
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture

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

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(1280./2.0, 720./2.0, 1.0); //TODO: increase unscaled res? - more tiles on screen

    world
        .create_entity()
        .with(Camera::standard_2d(1280., 720.)) // * width is halved in spritesheet.ron                                   
        .with(transform)
        .build();
}

fn initialise_world_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // ground is the first sprite in the sprite_sheet
    };
    for x in 0..50{
        for y in 0..50{
            transform.set_translation_xyz((x - y) as f32 * 32. , (x + y) as f32 * 17., 0.0);
            // screen.x = (map.x - map.y) * TILE_WIDTH_HALF;
            // screen.y = (map.x + map.y) * TILE_HEIGHT_HALF;
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { player: 0, tile_type: None})
                .with(TilePos{x, y})
                .with(Layer1)
                .build();
        }
    }
}

fn initialise_overlay_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle, 
        sprite_number: 4 // blank sprite
    };
    for x in 0..50{
        for y in 0..50{
            transform.set_translation_xyz((x - y) as f32 * 32. , (x + y) as f32 * 17., 0.00001); // z > 0 so it is displayed above layer 0
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { player: 0, tile_type: None})
                .with(TilePos{x, y})
                .with(Layer2)
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

// TODO: prefabs for other elements?
fn initialise_lower_menu(world: &mut World){
    //would use the UiButtonBuilder but that function seems to be broken
    //TODO: ? fix in a pr
    //seems that the .with_image() is broken and just the rest of the struct
    world.exec(|mut creator: UiCreator<'_>| {
        creator.create("ui/lower_panel.ron", ());
        creator.create("ui/build_menu.ron", ());
    });           
}

fn initialise_interact_menus(world: &mut World){
    //would use the UiButtonBuilder but that function seems to be broken
    //TODO: ? fix in a pr
    //seems that the .with_image() is broken and just the rest of the struct
    world.exec(|mut creator: UiCreator<'_>| {
        creator.create("ui/barracks_menu.ron", ());
    });           
}

fn initialise_follow_ent(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) { // * inits the entity that will follow the mouse when tile_mouse_follow is running
    let transform = Transform::default();                                // * this is not part of the sheet, thats so I dont have to constantly r&w to the sheet
                                                                                        // * store what was on the tile and replace it when the mouse moves etc etc
                                                                                        // * this way I only move a sprite to the translation of a tile instead of on the sheet
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
