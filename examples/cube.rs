#![feature(inherent_associated_types)]

use bevy::prelude::*;
use bevy_bundled::{Bundled, ResourceBundle};

#[derive(Bundled)]
struct SceneStyle {
    cube_speed: f32,
    light_speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (light_move, cube_bounce))
        .run();
}

#[derive(Component)]
struct Cube;

#[derive(Component)]
struct Light;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Cube,
    ));
    // light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        Light,
    ));
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(
        SceneStyle {
            cube_speed: 2.0,
            light_speed: 1.0,
        }
        .bundled(),
    );
}

fn cube_bounce(
    time: Res<Time>,
    speed: Query<&SceneStyle::CubeSpeed>,
    mut query: Query<&mut Transform, With<Cube>>,
) {
    let elapsed = time.elapsed().as_secs_f32();

    let speed = speed.single().0;

    *query.single_mut() = Transform::from_xyz(0.0, 1.5 + (elapsed * speed).sin(), 0.0);
}

fn light_move(
    time: Res<Time>,
    speed: Query<&SceneStyle::LightSpeed>,
    mut query: Query<&mut Transform, With<Light>>,
) {
    let elapsed = time.elapsed().as_secs_f32();
    let speed = speed.single().0;

    *query.single_mut() = Transform::from_xyz(
        4.0 * (elapsed * speed).sin(),
        8.0,
        4.0 * (elapsed * speed).cos(),
    );
}
