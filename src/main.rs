use bevy::prelude::*;
use bevy::camera::ScalingMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Crystal Space Iso".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 80.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_crystal)
        .run();
}

#[derive(Component)]
struct CrystalSphere;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::FixedVertical { viewport_height: 10.0 },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            range: 20.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
    let sphere_mesh = meshes.add(Mesh::from(Sphere::new(1.0)));
    let crystal_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.5, 1.0),
        perceptual_roughness: 0.05,
        metallic: 0.0,
        //specular_transmission: 0.95,
        ior: 1.5,
        thickness: 1.0,
        ..default()
    });
    commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(crystal_material),
        Transform::from_xyz(0.0, 0.5, 0.0),
        CrystalSphere,
    ));
    let floor_mesh = meshes.add(Mesh::from(Rectangle::new(10.0, 10.0)));
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        perceptual_roughness: 0.8,
        ..default()
    });
    commands.spawn((
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_material),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -1.0, 0.0)),
    ));
}

fn rotate_crystal(time: Res<Time>, mut query: Query<&mut Transform, With<CrystalSphere>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.5 * time.delta_secs());
        //transform.translation.y = 0.5 + (time.elapsed_secs() * 2.0).sin() * 0.1;
    }
}
