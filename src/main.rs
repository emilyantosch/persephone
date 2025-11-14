use std::time::Duration;

use bevy::{post_process::bloom::Bloom, prelude::*};

use crate::player::PlayerPlugin;

const PLAYER_SPEED: f32 = 100.;
const CAMERA_DECAY_RATE: f32 = 2.;

pub mod player;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerDashSpeed(f32);

#[derive(Component)]
struct PlayerDashDuration(Duration);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, (setup_scene, setup_camera))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.3, 0.2))),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}
