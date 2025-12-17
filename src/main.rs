use bevy::prelude::*;
use bevy::camera::ScalingMode;
use rand::Rng; // Import rand for generating random planets

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Crystal Space Iso".into(),
                ..default()
            }),
            ..default()
        }))
        // Space background color (dark)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.05)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 20.0, // Dim ambient, rely on the star's light
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_objects)
        .run();
}

#[derive(Component)]
struct Rotator {
    speed: f32,
}

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

    // 1. Camera - Zoomed out a bit to see the system
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::FixedVertical { viewport_height: 25.0 },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2. The Star (Center of the system)
    let star_mesh = meshes.add(Mesh::from(Sphere::new(2.0)));
    let star_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.2), // Yellow/Orange
        emissive: LinearRgba::new(10.0, 8.0, 2.0, 1.0), // Glows
        ..default()
    });

    commands.spawn((
        Mesh3d(star_mesh),
        MeshMaterial3d(star_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        // Add a light source to the star
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            radius: 2.0,
            ..default()
        },
    ));

    // 3. Generate Planets
    let num_planets = 4;
    let station_planet_index = rng.random_range(0..num_planets); // Pick a random planet for the station

    for i in 0..num_planets {
        let orbit_radius = 6.0 + (i as f32 * 4.0); // Distribute planets
        let orbit_speed = rng.random_range(0.2..0.8) * if i % 2 == 0 { 1.0 } else { -1.0 };
        let planet_size = rng.random_range(0.5..1.2);
        
        let planet_color = Color::srgb(
            rng.random::<f32>(),
            rng.random::<f32>(),
            rng.random::<f32>(),
        );

        // Planet Pivot (Center of the system, rotates to create orbit)
        commands.spawn((
            Transform::default(),
            Visibility::default(),
            Rotator { speed: orbit_speed },
        )).with_children(|parent| {
            // Planet Anchor (Offset by radius, holds the actual planet mesh)
            parent.spawn((
                Transform::from_xyz(orbit_radius, 0.0, 0.0),
                Visibility::default(),
            )).with_children(|planet_anchor| {
                
                // The Planet Mesh itself
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

                // 4. Spawn Station (if this is the chosen planet)
                if i == station_planet_index {
                    // Station Pivot (Rotates around the planet)
                    planet_anchor.spawn((
                        Transform::default(),
                        Visibility::default(),
                        Rotator { speed: 2.0 }, // Fast orbit around planet
                    )).with_children(|station_pivot| {
                        
                        // Station Mesh
                        let station_mesh = meshes.add(Mesh::from(Cuboid::new(0.4, 0.2, 0.4)));
                        let station_mat = materials.add(StandardMaterial {
                            base_color: Color::srgb(0.8, 0.8, 0.9),
                            metallic: 0.8,
                            perceptual_roughness: 0.2,
                            ..default()
                        });

                        station_pivot.spawn((
                            Mesh3d(station_mesh),
                            MeshMaterial3d(station_mat),
                            Transform::from_xyz(planet_size + 1.0, 0.0, 0.0), // Orbit slightly outside planet
                            Station,
                        ));
                    });
                }
            });
        });
    }
}

// Universal system to rotate anything with a Rotator component
fn rotate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Rotator)>) {
    for (mut transform, rotator) in &mut query {
        transform.rotate_y(rotator.speed * time.delta_secs());
    }
}
