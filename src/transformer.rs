use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::player::{Player, SquarePos};

const FILL_COLOR: Color = Color::Rgba {
    red: 0.9137,
    green: 0.1176,
    blue: 0.3882,
    alpha: 1.0,
};

const STROKE_COLOR: Color = Color::Rgba {
    red: 0.9255,
    green: 0.2510,
    blue: 0.4784,
    alpha: 1.0,
};

#[derive(Copy, Clone)]
pub enum Transformation {
    AddRight,
    AddTop,
    RotateCw,
}

impl Transformation {
    fn apply(&self, player: &mut Player) {
        let (w, h) = player.get_dimens();
        match self {
            Transformation::AddRight => {
                player.add_square(w, 0);
            },
            Transformation::AddTop => {
                player.add_square(0, h);
            },
            Transformation::RotateCw => {
                for square in &mut player.squares {
                    *square = SquarePos(square.1, h - square.0 - 1);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Transformer {
    position: Vec2,
    radius: f32,
    transformation: Transformation,
}

impl Transformer {
    fn new(x: f32, y: f32, transformation: Transformation) -> Self {
        Transformer {
            position: Vec2::new(x, y),
            radius: 30.0,
            transformation,
        }
    }

    fn get_shape(&self) -> shapes::Circle {
        shapes::Circle {
            radius: self.radius,
            center: self.position,
        }
    }
}


#[derive(Bundle)]
pub struct TransformerBundle {
    shape_bundle: ShapeBundle,
    fill: Fill,
    stroke: Stroke,
    transformer: Transformer,
}

impl TransformerBundle {
    pub fn new(x: f32, y: f32, transformation: Transformation) -> Self {
        let transformer = Transformer::new(x, y, transformation);
        let shape = transformer.get_shape();
        TransformerBundle {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            },
            fill: Fill::color(FILL_COLOR),
            stroke: Stroke::new(STROKE_COLOR, 10.0),
            transformer,
        }
    }
}


pub fn apply_transformations(
    mut player_info: Query<(&mut Player, &mut Collider, &mut Path, &Transform)>,
    transformers: Query<&Transformer>,
) {
    for (mut player, mut collider, mut path, player_transform) in &mut player_info {
        let mut collided_with_transformer = false;
        for transformer in &transformers {
            let distance = collider.distance_to_point(
                Vec2::new(player_transform.translation.x, player_transform.translation.y),
                0.0,
                transformer.position,
                true
            );
            if distance < transformer.radius {
                if !player.is_being_transformed {
                    player.is_being_transformed = true;
                    transformer.transformation.apply(&mut player);
                    *path = player.get_shape();
                    *collider = player.get_collider();
                }
                collided_with_transformer = true;
            }
        }
        if !collided_with_transformer {
            player.is_being_transformed = false;
        }
    }
}
