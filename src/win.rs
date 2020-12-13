use amethyst::{
    assets::Loader,
    core::{transform::Transform},
    ecs::prelude::Entity,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform, LineMode},
    input::{VirtualKeyCode, is_key_down},
};


pub struct Win {
    pub winner : i32,
}

pub struct MenuElems {
    pub title: Entity,
}

impl SimpleState for Win {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        world.delete_all();
        initialise_camera(world);
        initialise_menuelems(world);
        modify_text(world, self.winner)
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans{
        Trans::None
    }


    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_menuelems(world);
    }
}

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
        format!(""),
        [1., 1., 1., 1.],
        50.,
        LineMode::Single,
        Anchor::TopMiddle,
    ))
    .build();


    world.insert(MenuElems {title});
}

fn modify_text(world : &mut World, winner : i32){
    let mut ui_text = world.write_storage::<UiText>();
    if let Some(text) = ui_text.get_mut(world.write_resource::<MenuElems>().title) {
        text.text = format!("Player {} WON!", winner);
    }
}
