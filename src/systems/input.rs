use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;
use bevy::camera::ScalingMode;
use crate::components::Planet;

pub fn zoom_camera(
    mut events: MessageReader<MouseWheel>,
    mut query: Query<(&mut Projection, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();
    let zoom_speed = 0.5;
    let min_zoom = 0.2;
    let max_zoom = 5.0;
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let window_size = Vec2::new(window.width(), window.height());
    for event in events.read() {
        for (mut projection, mut transform) in query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                let old_scale = ortho.scale;
                let mut target_scale = old_scale - event.y * zoom_speed * 0.2;
                target_scale = target_scale.clamp(min_zoom, max_zoom);
                let scale_diff = old_scale - target_scale;
                if scale_diff.abs() < 0.00001 {
                    continue;
                }
                let ndc_x = (cursor_pos.x / window_size.x) * 2.0 - 1.0;
                let ndc_y = -((cursor_pos.y / window_size.y) * 2.0 - 1.0);
                let view_height = match ortho.scaling_mode {
                    ScalingMode::FixedVertical { viewport_height } => viewport_height,
                    ScalingMode::FixedHorizontal { viewport_width } => viewport_width / (window_size.x / window_size.y),
                    _ => 25.0, 
                };
                let view_width = view_height * (window_size.x / window_size.y);
                let offset_x = ndc_x * (view_width / 2.0);
                let offset_y = ndc_y * (view_height / 2.0);
                let right = transform.right();
                let up = transform.up();
                let shift = (right * offset_x + up * offset_y) * scale_diff;
                transform.translation += shift;
                ortho.scale = target_scale;
            }
        }
    }
}

pub fn pan_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &Projection), With<Camera>>,
) {
    let window = window_query.single().unwrap();
    let mut pan = Vec2::ZERO;
    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyK]) {
        pan.y += 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyJ]) {
        pan.y -= 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyH]) {
        pan.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyL]) {
        pan.x += 1.0;
    }
    if let Some(cursor_pos) = window.cursor_position() {
        let margin = 20.0;
        let width = window.width();
        let height = window.height();
        if cursor_pos.x < margin {
            pan.x -= 1.0;
        } else if cursor_pos.x > width - margin {
            pan.x += 1.0;
        }
        if cursor_pos.y < margin {
            pan.y += 1.0;
        } else if cursor_pos.y > height - margin {
            pan.y -= 1.0;
        }
    }
    if pan != Vec2::ZERO {
        let pan_speed = 30.0; 
        for (mut transform, projection) in &mut query {
            if let Projection::Orthographic(ortho) = projection {
                let movement = pan.normalize_or_zero() * pan_speed * ortho.scale * time.delta_secs();
                let right = transform.right();
                let up = transform.up();
                transform.translation += right * movement.x + up * movement.y;
            }
        }
    }
}

pub fn constrain_camera(
    mut camera_query: Query<(&mut Transform, &Projection), With<Camera>>,
    planet_query: Query<&GlobalTransform, With<Planet>>,
) {
    let max_radius = planet_query.iter()
        .map(|t| t.translation().length())
        .fold(20.0, f32::max);
    for (mut transform, projection) in &mut camera_query {
        if let Projection::Orthographic(_) = projection {
            if transform.translation.y < 5.0 {
                transform.translation.y = 5.0;
            }
            let forward = transform.forward();
            if forward.y.abs() > 0.001 {
                let t = -transform.translation.y / forward.y;
                let ground_point = transform.translation + forward * t;
                let dist_sq = ground_point.xz().length_squared();
                let limit = max_radius + 5.0;
                if dist_sq > limit * limit {
                    let direction = ground_point.xz().normalize_or_zero();
                    let clamped_ground_xz = direction * limit;
                    let clamped_ground = Vec3::new(clamped_ground_xz.x, 0.0, clamped_ground_xz.y);
                    let correction = clamped_ground - ground_point;
                    transform.translation += correction;
                }
            }
        }
    }
}
