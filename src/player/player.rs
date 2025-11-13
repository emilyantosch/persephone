use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
    dash_speed: f32,
    player_dash_duration: Duration,
}
