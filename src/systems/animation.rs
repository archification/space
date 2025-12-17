use bevy::prelude::*;
use crate::components::{Rotator, Star};

pub fn rotate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Rotator)>) {
    for (mut transform, rotator) in &mut query {
        transform.rotate_y(rotator.speed * time.delta_secs());
    }
}

pub fn pulse_star(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&MeshMaterial3d<StandardMaterial>, With<Star>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(&handle.0) {
            let pulse = (time.elapsed_secs() * 2.0).sin() * 0.2 + 1.0;
            let current_color = material.base_color.to_linear();
            material.emissive = LinearRgba::new(
                current_color.red * 10.0 * pulse,
                current_color.green * 10.0 * pulse,
                current_color.blue * 10.0 * pulse,
                1.0
            );
        }
    }
}
