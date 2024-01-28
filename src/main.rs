mod player;
mod map;
mod transformer;
mod cave;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use player::{Player, spawn_player, spawn_player_at_point, move_player};
use transformer::apply_transformations;
use map::{spawn_map, next_level, Level, get_levels, start_level, button_system};

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

#[derive(Resource)]
pub struct LevelTransitioning(bool);

#[derive(Component)]
pub struct FadeToBlack {
    timer: Timer,
    switched_level: bool,
}

const BLACK: Color = Color::Rgba {
    red: 0.14453125,
    green: 0.125,
    blue: 0.19140625,
    alpha: 0.0,
};

fn fade_step(
    mut commands: Commands,
    mut fade_to_blacks: Query<(Entity, &mut Sprite, &mut FadeToBlack)>,
    asset_server: Res<AssetServer>,
    levels: Query<&Level>,
    level_entities: Query<Entity, With<Level>>,
    player_entities: Query<Entity, With<Player>>,
    mut level_transitioning: ResMut<LevelTransitioning>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut fade_to_black) in &mut fade_to_blacks {
        fade_to_black.timer.tick(time.delta());

        if fade_to_black.timer.percent() < 0.4 {
            sprite.color.set_a(fade_to_black.timer.percent() / 0.4);
        } else if fade_to_black.timer.percent() > 0.6 {
            sprite.color.set_a(fade_to_black.timer.percent_left() / 0.4);
        } else {
            sprite.color.set_a(1.0);
        }

        if fade_to_black.timer.percent() > 0.4 && !fade_to_black.switched_level {
            let current_level = levels.iter().next().unwrap().levelid;
            fade_to_black.switched_level = true;
            for entity in &level_entities {
                commands.entity(entity).despawn();
            }
            start_level(&mut commands, &asset_server, current_level + 1);
            for entity in &player_entities {
                commands.entity(entity).despawn();
            }
            if current_level != 0 {
                spawn_player_at_point(&mut commands, get_levels()[current_level + 1].spawn_point);
            }
        }

        if fade_to_black.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
            level_transitioning.0 = false;
        }
    }
}

pub fn spawn_fade_to_black(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BLACK,
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 99.0),
            ..default()
        },
        FadeToBlack {
            timer: Timer::from_seconds(1., TimerMode::Once),
            switched_level: false,
        },
    ));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::new(0.0, -100.0);
    commands.spawn(Camera2dBundle::default());
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/chained_to_a_cloud_2.wav"),
        settings: PlaybackSettings::LOOP,
    });
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
        spawn_player_at_point(&mut commands, spawn_point);
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
            fade_step,
            button_system,
        ))
        .insert_resource(LevelTransitioning(false))
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
