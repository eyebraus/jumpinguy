use bevy::{
    prelude::{App, DefaultPlugins, PluginGroup},
    window::{Window, WindowMode, WindowPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                title: "jumpinguy".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
