use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::player::{Player, SquarePos};

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

    fn get_sprite_path(&self) -> &str {
        match self {
            Transformation::AddRight => "transformers/add_right.png",
            Transformation::AddTop => "transformers/add_top.png",
            Transformation::RotateCw => "transformers/rotate_cw.png",
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
}


#[derive(Bundle)]
pub struct TransformerBundle {
    sprite_bundle: SpriteBundle,
    transformer: Transformer,
}

impl TransformerBundle {
    pub fn new(x: f32, y: f32, transformation: Transformation, asset_server: &Res<AssetServer>) -> Self {
        let transformer = Transformer::new(x, y, transformation);
        TransformerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(transformation.get_sprite_path()),
                transform: Transform::from_xyz(x, y, -1.0),
                ..default()
            },
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
