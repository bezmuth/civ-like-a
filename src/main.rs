mod game;
mod menu;
mod systems;
mod components;

use crate::menu::Menu;

use amethyst::{
    core::{TransformBundle, HideHierarchySystemDesc},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use amethyst_imgui::RenderImgui;
// this is mostly boilerplate that sets up the amethyst window then switches to the main menu state.
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");

    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(
                app_root.join("config/bindings.ron"),
            )?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with SpriteRender component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderImgui::<StringBindings>::default())
                .with_plugin(RenderUi::default())

        )?
        .with_system_desc(HideHierarchySystemDesc, "hidehierarchysystemdesc", &[]); // lets all the widgets of a panel be hidden

    let mut game = Application::new(assets_dir, Menu::default(), game_data)?;
    game.run();
    Ok(())
}
