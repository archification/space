use bevy::prelude::*;
mod components;
mod systems;
use systems::{setup::setup, animation::{rotate_objects, pulse_star}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Crystal Space Iso".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.05)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 20.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_objects, pulse_star))
        .run();
}
