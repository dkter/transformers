mod player;
mod map;
mod transformer;
mod cave;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use player::{Player, spawn_player, spawn_player_at_point, move_player};
use transformer::apply_transformations;
use map::{spawn_map, next_level, Level};

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
    levels: Query<&Level>,
) {
    if keyboard_input.just_released(KeyCode::R) {
        let spawn_point = levels.iter().next().unwrap().spawn_point;
        for entity in &player_entities {
            commands.entity(entity).despawn();
        }
        spawn_player_at_point(commands, spawn_point);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, (setup, spawn_player, spawn_map))
        .add_systems(Update, (
            move_player,
            player::set_jumping_false,
            apply_transformations,
            check_restart,
            next_level,
        ))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "cool game".into(),
                    resolution: (1200., 800.).into(),
                    ..default()
                }),
                ..default()
            }),
            ShapePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .run();
}
