use std::time::Duration;

use bevy::{post_process::bloom::Bloom, prelude::*};

const PLAYER_SPEED: f32 = 100.;
const CAMERA_DECAY_RATE: f32 = 2.;

mod player;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerDashSpeed(f32);

#[derive(Component)]
struct PlayerDashDuration(Duration);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, setup_camera))
        .add_systems(
            Update,
            (
                move_player,
                dash,
                handle_dash.run_if(any_with_component::<ActiveDashCooldown>),
                animate_dash.run_if(any_with_component::<Ghost>),
                recover_dash,
            ),
        )
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
        DashCooldown(Timer::from_seconds(0.2, TimerMode::Once)),
        DashChargeCooldown(Timer::from_seconds(1.5, TimerMode::Once)),
        DashCharges {
            charges: 3,
            max_charges: 3,
        },
        PlayerDashSpeed(1.0),
        PlayerDashDuration(Duration::from_millis(100)),
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}

fn move_player(
    mut player: Single<(&mut Transform, &PlayerDashSpeed), With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * player.1.0 * time.delta_secs();
    player.0.translation += move_delta.extend(0.);
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct ActiveDashCooldown;

#[derive(Component)]
struct DashCooldown(Timer);

#[derive(Component)]
struct DashCharges {
    charges: i32,
    max_charges: i32,
}

#[derive(Component)]
struct DashChargeCooldown(Timer);

#[derive(Component)]
struct GhostDecayTimer(Timer);

#[derive(Component)]
struct Ghost;

fn dash(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut DashCooldown,
            &mut PlayerDashSpeed,
            Option<&ActiveDashCooldown>,
            &mut DashCharges,
        ),
        With<Player>,
    >,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    for (entity, mut dash_cooldown, mut player_dash_speed, active_cooldown, mut charges) in
        &mut query
    {
        if kb_input.pressed(KeyCode::ShiftLeft) && active_cooldown.is_none() && charges.charges > 0
        {
            dash_cooldown.0.reset();
            charges.charges -= 1;
            commands.entity(entity).insert(ActiveDashCooldown);
            player_dash_speed.0 = 80.0;
        }
    }
}

fn handle_dash(
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &PlayerDashDuration,
            &mut DashCooldown,
            &mut PlayerDashSpeed,
            &Transform,
            &Mesh2d,
            &MeshMaterial2d<ColorMaterial>,
        ),
        With<Player>,
    >,
    mut commands: Commands,
) {
    for (
        entity,
        player_dash_duration,
        mut dash_cooldown,
        mut player_dash_speed,
        transform,
        mesh,
        material,
    ) in &mut query
    {
        dash_cooldown.0.tick(time.delta());
        commands.spawn((
            Transform::from_translation(transform.translation),
            mesh.clone(),
            material.clone(),
            GhostDecayTimer(Timer::new(Duration::from_millis(100), TimerMode::Once)),
            Ghost,
        ));
        if dash_cooldown.0.just_finished() {
            commands.entity(entity).remove::<ActiveDashCooldown>();
        } else if dash_cooldown.0.elapsed() > player_dash_duration.0 {
            player_dash_speed.0 = 1.0;
        }
    }
}

fn animate_dash(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ghost_query: Query<
        (
            Entity,
            &mut GhostDecayTimer,
            &mut MeshMaterial2d<ColorMaterial>,
        ),
        With<Ghost>,
    >,
) {
    for (ghost_entity, mut ghost_decay_timer, mut material) in &mut ghost_query {
        ghost_decay_timer.0.tick(time.delta());
        if ghost_decay_timer.0.just_finished() {
            commands.entity(ghost_entity).despawn();
        } else {
            let alpha = 1.0
                - (ghost_decay_timer.0.elapsed_secs()
                    / ghost_decay_timer.0.duration().as_secs_f32());
            *material = MeshMaterial2d(materials.add(Color::srgba(1.0, 0.2, 0.3, alpha)));
        }
    }
}

fn recover_dash(
    time: Res<Time>,
    mut query: Query<(&mut DashChargeCooldown, &mut DashCharges), With<Player>>,
) {
    for (mut dash_charge_cooldown, mut dash_charges) in &mut query {
        if dash_charges.charges < dash_charges.max_charges {
            dash_charge_cooldown.0.tick(time.delta());
            if dash_charge_cooldown.0.just_finished() {
                dash_charges.charges += 1;
                dash_charge_cooldown.0.reset();
            }
        }
    }
}
