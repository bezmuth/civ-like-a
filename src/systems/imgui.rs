extern crate amethyst;
extern crate amethyst_imgui;
use amethyst::{
	input::{InputBundle, StringBindings},
	prelude::*,
	renderer::{bundle::RenderingBundle, types::DefaultBackend, RenderToWindow},
	utils::application_root_dir,
};

use amethyst_imgui::RenderImgui;

#[derive(Default, Clone, Copy)]
pub struct Imgui;
impl<'s> amethyst::ecs::System<'s> for Imgui {
	type SystemData = ();

	fn run(&mut self, _: Self::SystemData) {
		amethyst_imgui::with(|ui| {
			ui.show_demo_window(&mut true);
		});
	}
}
