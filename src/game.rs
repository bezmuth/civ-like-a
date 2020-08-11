use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{transform::Transform, ArcThreadPool},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shred::DispatcherBuilder,
    shred::Dispatcher, input::{is_key_down, VirtualKeyCode},
};

use super::systems;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
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
        //initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
        initialise_world_sheet(world, self.sprite_sheet_handle.clone().unwrap())
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Go back to the `Menu` state.
                data.world.delete_all();
                return Trans::Pop;
            }
        }
        // Escape isn't pressed, so we stay in this `State`.
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

pub struct Tiles{
    pub layer: i32,
}

impl Tiles {
    fn new(_layer: i32) -> Tiles {
        Tiles {
            layer: _layer
        }
    }
}

impl Component for Tiles {
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
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_world_sheet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    


    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // ground is the first sprite in the sprite_sheet
    };
    for x in (0..640).step_by(64){
        for y in (0..320).step_by(32){
            transform.set_translation_xyz(20. + x as f32, 20. +y as f32, 0.0);
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform.clone())
                .with(Tiles::new(0))
                .build();
        }
    }
    // world.insert(Tiles::new(tile1))
}

