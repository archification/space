use bevy::prelude::*;
use bevy::camera::ScalingMode;
use rand::Rng;

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

#[derive(Component)]
struct Rotator {
    speed: f32,
}

#[derive(Component)]
struct Star;

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct Station;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::FixedVertical { viewport_height: 25.0 },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let star_mesh = meshes.add(Mesh::from(Sphere::new(2.0)));
    let s_r = rng.random::<f32>();
    let s_g = rng.random::<f32>();
    let s_b = rng.random::<f32>();
    let star_material = materials.add(StandardMaterial {
        base_color: Color::srgb(s_r, s_g, s_b), // Yellow/Orange
        emissive: LinearRgba::new(s_r * 10.0, s_g * 10.0, s_b * 10.0, 1.0),
        ..default()
    });

    commands.spawn((
        Mesh3d(star_mesh),
        MeshMaterial3d(star_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Star,
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            radius: 2.0,
            color: Color::srgb(s_r, s_g, s_b),
            ..default()
        },
    ));
    let num_planets = 4;
    let station_planet_index = rng.random_range(0..num_planets);

    for i in 0..num_planets {
        let orbit_radius = 6.0 + (i as f32 * 4.0);
        let orbit_speed = rng.random_range(0.2..0.8) * if i % 2 == 0 { 1.0 } else { -1.0 };
        let planet_size = rng.random_range(0.5..1.2);
        
        let planet_color = Color::srgb(
            rng.random::<f32>(),
            rng.random::<f32>(),
            rng.random::<f32>(),
        );
        commands.spawn((
            Transform::default(),
            Visibility::default(),
            Rotator { speed: orbit_speed },
        )).with_children(|parent| {
            parent.spawn((
                Transform::from_xyz(orbit_radius, 0.0, 0.0),
                Visibility::default(),
            )).with_children(|planet_anchor| {
                let planet_mesh = meshes.add(Mesh::from(Sphere::new(planet_size)));
                let planet_mat = materials.add(StandardMaterial {
                    base_color: planet_color,
                    perceptual_roughness: 0.8,
                    ..default()
                });
                planet_anchor.spawn((
                    Mesh3d(planet_mesh),
                    MeshMaterial3d(planet_mat),
                    Planet,
                ));
                if i == station_planet_index {
                    planet_anchor.spawn((
                        Transform::default(),
                        Visibility::default(),
                        Rotator { speed: 2.0 },
                    )).with_children(|station_pivot| {
                        let station_mesh = meshes.add(Mesh::from(Cuboid::new(0.4, 0.2, 0.4)));
                        let station_mat = materials.add(StandardMaterial {
                            base_color: Color::srgb(0.8, 0.8, 0.9),
                            metallic: 0.8,
                            perceptual_roughness: 0.2,
                            ..default()
                        });
                        let window_mat = materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            emissive: LinearRgba::new(50.0, 40.0, 10.0, 1.0),
                            ..default()
                        });
                        let window_mesh = meshes.add(Mesh::from(Cuboid::new(0.05, 0.1, 0.02)));
                        let ray_mat = materials.add(StandardMaterial {
                            base_color: Color::hsla(45.0, 1.0, 0.8, 0.05),
                            alpha_mode: AlphaMode::Add,
                            unlit: true,
                            ..default()
                        });
                        let ray_height = 0.8;
                        let ray_mesh = meshes.add(Mesh::from(Cone {
                            radius: 0.25,
                            height: ray_height
                        }));
                        station_pivot.spawn((
                            Mesh3d(station_mesh),
                            MeshMaterial3d(station_mat),
                            Transform::from_xyz(planet_size + 1.0, 0.0, 0.0),
                            Station,
                        )).with_children(|station| {
                            station.spawn((
                                PointLight {
                                    color: Color::srgb(0.0, 1.0, 1.0),
                                        intensity: 2_000.0,
                                        range: 30.0,
                                        shadows_enabled: true,
                                        radius: 0.1,
                                        ..default()
                                },
                                Transform::from_xyz(0.0, 0.3, 0.0),
                            ));
                            let side_offset = 0.201;
                            let window_z_positions = [-0.1, 0.1];
                            for z in window_z_positions {
                                station.spawn((
                                    Mesh3d(window_mesh.clone()),
                                    MeshMaterial3d(window_mat.clone()),
                                    Transform::from_xyz(side_offset, 0.0, z),
                                )).with_children(|win| {
                                    win.spawn((
                                        Mesh3d(ray_mesh.clone()),
                                            MeshMaterial3d(ray_mat.clone()),
                                            Transform::from_xyz(ray_height / 2.0, 0.0, 0.0)
                                                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                                    ));
                                });
                            }
                            for z in window_z_positions {
                                station.spawn((
                                    Mesh3d(window_mesh.clone()),
                                    MeshMaterial3d(window_mat.clone()),
                                    Transform::from_xyz(-side_offset, 0.0, z),
                                )).with_children(|win| {
                                    win.spawn((
                                        Mesh3d(ray_mesh.clone()),
                                        MeshMaterial3d(ray_mat.clone()),
                                        Transform::from_xyz(-ray_height / 2.0, 0.0, 0.0)
                                            .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                                    ));
                                });
                            }
                        });
                    });
                }
            });
        });
    }
}

fn rotate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Rotator)>) {
    for (mut transform, rotator) in &mut query {
        transform.rotate_y(rotator.speed * time.delta_secs());
    }
}

fn pulse_star(
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
