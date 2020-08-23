use amethyst::{
    assets::Loader,
    core::{transform::Transform},
    ecs::prelude::Entity,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform, LineMode},
    input::{VirtualKeyCode, is_key_down},
};

use crate::game::Civ;


#[derive(Default)]
pub struct Menu {
}

pub struct MenuElems {
    pub title: Entity,
    pub lower_text: Entity,
}

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        initialise_menuelems(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                // Go to the `Game` State.
                data.world.delete_all();
                return Trans::Push(Box::new(Civ::default()));
            }
        }
        // Space isn't pressed, so we stay in this `State`.
        Trans::None
    }

    // This code tells Amethyst to run all the systems in your game data.
    // fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    //     data.data.update(&data.world);
    //     Trans::None
    // }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_menuelems(world);
    }
}

/// Initialise the camera. // TODO: reimplement camera
fn initialise_camera(world: &mut World) {
    let transform = Transform::default();

    world
        .create_entity()
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
    let title_info = UiTransform::new(
        "title".to_string(),
        Anchor::TopMiddle, 
        Anchor::Middle,
        0., 
        -30., 
        0., 
        600., 
        50.,
    );

    let title = world
    .create_entity()
    .with(title_info)
    .with(UiText::new(
        font.clone(),
        format!("CivLike V.{}", env!("CARGO_PKG_VERSION")),
        [1., 1., 1., 1.],
        50.,
        LineMode::Single,
        Anchor::TopMiddle,
    ))
    .build();

    let lower_info = UiTransform::new(
        "lower".to_string(),
        Anchor::BottomMiddle, 
        Anchor::Middle,
        0., 
        30., 
        0., 
        200., 
        50.,
    );

    let lower_text = world
    .create_entity()
    .with(lower_info)
    .with(UiText::new(
        font.clone(),
        "Press SPACE to start".to_string(),
        [1., 1., 1., 1.],
        15.,
        LineMode::Single,
        Anchor::TopMiddle,
    ))
    .build();

    world.insert(MenuElems {title, lower_text});
}
