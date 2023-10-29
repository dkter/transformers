use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::player::{PLAYER_WIDTH, PLAYER_HEIGHT, SquarePos};

const INNER_COLOR: Color = Color::Rgba {
    red: 0.9765625,
    green: 0.94140625,
    blue: 0.421875,
    alpha: 1.0,
};

#[derive(Component, Clone)]
pub struct Cave {
    pub position: Vec2,
    pub squares: Vec<SquarePos>,
}

impl Cave {
    fn get_dimens(&self) -> (i32, i32) {
        let mut max_x = 0;
        let mut max_y = 0;
        for square in &self.squares {
            if square.0 + 1 > max_x {
                max_x = square.0 + 1;
            }
            if square.1 + 1 > max_y {
                max_y = square.1 + 1;
            }
        }
        (max_x, max_y)
    }

    pub fn get_shape(&self) -> Path {
        let mut builder = GeometryBuilder::new();
        for square in &self.squares {
            builder = builder.add(&shapes::Rectangle {
                extents: Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
                origin: RectangleOrigin::CustomCenter(Vec2::new(PLAYER_WIDTH * square.0 as f32, PLAYER_HEIGHT * square.1 as f32)),
            });
        }
        builder.build()
    }
}

#[derive(Bundle)]
pub struct CaveBundle {
    shape_bundle: ShapeBundle,
    fill: Fill,
    cave: Cave,
}

impl CaveBundle {
    pub fn new(cave: Cave) -> Self {
        let shape = cave.get_shape();
        let (w, h) = cave.get_dimens();
        CaveBundle {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_xyz(
                    cave.position.x - w as f32 * PLAYER_WIDTH / 2.0,
                    cave.position.y - h as f32 * PLAYER_HEIGHT / 2.0,
                    -1.0
                ),
                ..default()
            },
            fill: Fill::color(INNER_COLOR),
            cave,
        }
    }
}
