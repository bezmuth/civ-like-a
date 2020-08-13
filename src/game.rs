use std::time::Duration;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        transform::Transform, ArcThreadPool,
        frame_limiter::{FrameLimiter, FrameRateLimitStrategy},
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shred::DispatcherBuilder,
    shred::Dispatcher, input::{is_key_down, VirtualKeyCode},
};

use super::systems;
pub use super::components::{Tiles, Player, Build, Building, BuildingType};

#[derive(Default)]
pub struct Civ<'a, 'b> {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for Civ<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Game Start!");
        let world = data.world;
        // Create the `DispatcherBuilder` and register some `System`s that should only run for this `State`.
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(systems::SheetSystem, "sheet_system", &[]);
        dispatcher_builder.add(systems::CameraSystem{multiplier:1.}, "camera_system", &["sheet_system"]);
        dispatcher_builder.add(systems::BuildSystem, "build_system", &["sheet_system", "camera_system"]);
        dispatcher_builder.add(systems::ResourceCalcSystem, "resourcecalc_system", &["sheet_system", "camera_system", "build_system"]);
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
        // https://github.com/amethyst/amethyst/blob/8e8bc94867f96feeeb392dd1ab1564a0f1f8ed70/src/app.rs
        world.insert(FrameLimiter::new(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(14)), 60)); // 16.8 ms in a frame, 14 ms for sleep about 2 ms for yeild


        //TODO: Implement a resource initalise that loads a bunch of static info into thw world
        initialise_camera(world);
        // TODO: move all these to their own init?
        world.insert(Build {mode: BuildingType::None});
        world.insert(CurrentPlayer{ playernum: 0}); // * WORLD.INSERT WORKS FINE WITH RESOURCES
        world // * WORLD.INSERT DOES NOT WORK WITH COMPONENTS
            .create_entity()
            .with(Player::new(0)) // * width is halved in spritesheet.ron                                   
            .build();

        initialise_world_sheet(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_overlay_sheet(world, self.sprite_sheet_handle.clone().unwrap());
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
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

//TODO: alterantive to player: i8? (like a reference to player)
pub struct CurrentPlayer{
    pub playernum: i8,
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
    transform.set_translation_xyz(768./2.0, 432./2.0, 1.0); //TODO: increase unscaled res? - more tiles on screen

    world
        .create_entity()
        .with(Camera::standard_2d(768., 432.)) // * width is halved in spritesheet.ron                                   
        .with(transform)
        .build();
}

fn initialise_world_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // ground is the first sprite in the sprite_sheet
    };
    for x in 0..20{
        for y in 0..20{
            transform.set_translation_xyz((x - y) as f32 * 32. , (x + y) as f32 * 17., 0.0);
            // screen.x = (map.x - map.y) * TILE_WIDTH_HALF;
            // screen.y = (map.x + map.y) * TILE_HEIGHT_HALF;
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { layer: 0, player: 0, buildingtype: BuildingType::None, x, y})
                .build();
        }
    }
}

fn initialise_overlay_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle, 
        sprite_number: 2 // blank sprite
    };
    for x in 0..20{
        for y in 0..20{
            transform.set_translation_xyz((x - y) as f32 * 32. , (x + y) as f32 * 17., 0.00001); // z > 0 so it is displayed above layer 0
            // screen.x = (map.x - map.y) * TILE_WIDTH_HALF;
            // screen.y = (map.x + map.y) * TILE_HEIGHT_HALF;
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles { layer: 1, player: 0, buildingtype: BuildingType::None, x, y})
                .build();
        }
    }
}
