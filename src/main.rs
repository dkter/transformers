mod player;
mod map;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::{Player, spawn_player, move_player};
use map::spawn_map;

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::new(0.0, -100.0);
    commands.spawn(Camera2dBundle::default());
}

fn check_restart(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_entities: Query<Entity, With<Player>>,
) {
    if keyboard_input.just_released(KeyCode::R) {
        for entity in &player_entities {
            commands.entity(entity).despawn();
        }
        spawn_player(commands);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, (setup, spawn_player, spawn_map))
        .add_systems(Update, (move_player, player::set_jumping_false, check_restart))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .run();
}
