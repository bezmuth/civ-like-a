use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
    input::{VirtualKeyCode, is_key_down},
    EventReader,
};

use crate::game::Pong;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

#[derive(Default)]
pub struct Menu {
}

pub struct MenuElems {
    pub Title: Entity,
}

    

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        initialise_menuelems(world);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                // Go to the `Game` State.
                return Trans::Push(Box::new(Pong::default()));
            }
        }

        // Space isn't pressed, so we stay in this `State`.
        Trans::None
    }

    // This code tells Amethyst to run all the systems in your game data.
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        data.data.update(&data.world);
        Trans::None
    }
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

fn initialise_menuelems(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let titleInfo = UiTransform::new(
        "title".to_string(),
        Anchor::TopMiddle, 
        Anchor::Middle,
        0., 
        -30., 
        0., 
        200., 
        50.,
    );

    let title = world
    .create_entity()
    .with(titleInfo)
    .with(UiText::new(
        font.clone(),
        "CivLike".to_string(),
        [1., 1., 1., 1.],
        50.,
    ))
    .build();

    world.insert(MenuElems {Title: title});
}
