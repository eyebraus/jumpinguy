use avian2d::PhysicsPlugins;
use bevy::{
    asset::AssetPlugin,
    prelude::{App, DefaultPlugins, PluginGroup},
    window::{Window, WindowMode, WindowPlugin},
};

use jumpinguy_lib::{
    animation::AnimationPlugin, character::CharacterPlugin, sheet::SheetPlugin, state::StatePlugin,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        title: "jumpinguy".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(AnimationPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(SheetPlugin)
        .add_plugins(StatePlugin)
        .run();
}
