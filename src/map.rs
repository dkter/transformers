use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player::SquarePos;
use crate::transformer::{TransformerBundle, Transformation};
use crate::cave::{Cave, CaveBundle};

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

#[derive(Component)]
struct Level(usize);

struct LevelData {
    blocks: Vec<Block>,
    transformers: Vec<(f32, f32, Transformation)>,
    cave: Cave,
}

fn get_levels() -> Vec<LevelData> {
    vec![
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
            }
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
            }
        },
    ]
}

pub fn start_level(mut commands: Commands, levelid: usize) {
    let levels = get_levels();
    let level = &levels[levelid];
    for block in &level.blocks {
        commands.spawn((
            block.to_sprite_bundle(),
            block.to_collider(),
            Level(levelid),
        ));
    }
    for transformer_args in &level.transformers {
        let (x, y, transformation) = transformer_args;
        commands.spawn((
            TransformerBundle::new(*x, *y, *transformation),
            Level(levelid),
        ));
    }
    commands.spawn((
        CaveBundle::new(level.cave.clone()),
        Level(levelid),
    ));
}

pub fn spawn_map(commands: Commands) {
    start_level(commands, 0);
}
