use bevy::{post_process::bloom::Bloom, prelude::*};

const PLAYER_SPEED: f32 = 100.;
const CAMERA_DECAY_RATE: f32 = 2.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, setup_camera))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.3, 0.2))),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(100.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.2, 0.3))),
        Player,
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}
