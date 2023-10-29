use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player::{Player, PLAYER_WIDTH, PLAYER_HEIGHT, SquarePos};
use crate::transformer::{TransformerBundle, Transformation};
use crate::cave::{Cave, CaveBundle};
use crate::{spawn_fade_to_black, LevelTransitioning};

struct Block {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Block {
    fn x_ctr(&self) -> f32 {
        self.x + self.w / 2.0
    }

    fn y_ctr(&self) -> f32 {
        self.y - self.h / 2.0
    }

    fn to_sprite_bundle(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(self.w, self.h)),
                ..default()
            },
            transform: Transform::from_xyz(self.x_ctr(), self.y_ctr(), 0.0),
            ..default()
        }
    }

    fn to_collider(&self) -> Collider {
        Collider::cuboid(self.w / 2.0, self.h / 2.0)
    }
}

#[derive(Component, Copy, Clone)]
pub struct Level {
    pub levelid: usize,
    pub spawn_point: (f32, f32),
}

pub struct LevelData {
    blocks: Vec<Block>,
    transformers: Vec<(f32, f32, Transformation)>,
    cave: Cave,
    background: Option<String>,
    pub spawn_point: (f32, f32),
}

pub fn get_levels() -> Vec<LevelData> {
    vec![
        LevelData {
            blocks: vec![
                // frame
                Block { x: -600.0, y: 400.0, w: 50.0, h: 800.0 },
                Block { x: -600.0, y: -350.0, w: 1200.0, h: 50.0 },
                Block { x: -600.0, y: 400.0, w: 1200.0, h: 50.0 },
                Block { x: 550.0, y: 400.0, w: 50.0, h: 800.0 },
                // pole
                Block { x: 300.0, y: -100.0, w: 50.0, h: 250.0 },
                // misc platforms
                Block { x: -100.0, y: -250.0, w: 100.0, h: 50.0 },
                Block { x: -250.0, y: -200.0, w: 100.0, h: 50.0 },
                Block { x: -450.0, y: -100.0, w: 250.0, h: 50.0 },
                Block { x: -450.0, y: -50.0, w: 50.0, h: 50.0 },
                Block { x: -350.0, y: 50.0, w: 50.0, h: 50.0 },
                Block { x: -300.0, y: 150.0, w: 50.0, h: 50.0 },
                Block { x: -250.0, y: 250.0, w: 200.0, h: 50.0 },
            ],
            transformers: vec![],
            cave: Cave {
                position: Vec2::new(500.0, -250.0),
                squares: vec![SquarePos(0, 0)],
            },
            background: Some(String::from("backgrounds/level0.png")),
            spawn_point: (-550.0, -200.0),
        },
        LevelData {
            blocks: vec![
                Block { x: -400.0, y: -200.0, w: 800.0, h: 50.0 },
                Block { x: 200.0, y: -100.0, w: 50.0, h: 100.0 },
            ],
            transformers: vec![
                (-100.0, -175.0, Transformation::AddRight),
            ],
            cave: Cave {
                position: Vec2::new(-350.0, -150.0),
                squares: vec![SquarePos(0, 0)],
            },
            background: None,
            spawn_point: (0.0, 0.0),
        },
        LevelData {
            blocks: vec![
                Block { x: -300.0, y: -300.0, w: 600.0, h: 50.0 },
            ],
            transformers: vec![
                (-100.0, -275.0, Transformation::AddRight),
            ],
            cave: Cave {
                position: Vec2::new(-400.0, -200.0),
                squares: vec![SquarePos(0, 0)],
            },
            background: None,
            spawn_point: (0.0, 0.0),
        },
    ]
}

pub fn start_level(commands: &mut Commands, asset_server: &Res<AssetServer>, levelid: usize) {
    let levels = get_levels();
    let level_data = &levels[levelid];
    let level = Level { levelid, spawn_point: level_data.spawn_point };
    if let Some(background_path) = &level_data.background {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(background_path),
                transform: Transform::from_xyz(0.0, 0.0, -2.0).with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            level,
        ));
    }
    for block in &level_data.blocks {
        commands.spawn((
            block.to_sprite_bundle(),
            block.to_collider(),
            level,
        ));
    }
    for transformer_args in &level_data.transformers {
        let (x, y, transformation) = transformer_args;
        commands.spawn((
            TransformerBundle::new(*x, *y, *transformation),
            level,
        ));
    }
    commands.spawn((
        CaveBundle::new(level_data.cave.clone()),
        level,
    ));
}

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    start_level(&mut commands, &asset_server, 0);
}

pub fn next_level(
    mut commands: Commands,
    player_info: Query<(&Player, &Transform)>,
    caves: Query<(&Cave, &Transform)>,
    mut level_transitioning: ResMut<LevelTransitioning>,
) {
    if level_transitioning.0 {
        return;
    }
    let mut new_level = false;
    for (player, player_transform) in &player_info {
        for (cave, cave_transform) in &caves {
            if (player_transform.translation.x - cave_transform.translation.x).abs() < PLAYER_WIDTH / 2.0
                    && (player_transform.translation.y - cave_transform.translation.y).abs() < PLAYER_HEIGHT / 2.0
                    && cave.matches_player(&player) {
                // new level
                new_level = true;
            }
        }
    }
    if new_level {
        level_transitioning.0 = true;
        spawn_fade_to_black(&mut commands);
    }
}
