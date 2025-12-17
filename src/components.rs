use bevy::prelude::*;

#[derive(Component)]
pub struct Rotator {
    pub speed: f32,
}

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Station;
