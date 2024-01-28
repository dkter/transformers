use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::player::{Player, SquarePos};

pub enum TransformerAnimState {
    NotAnimating,
    MovingToward { orig_pos: Vec3, transformer_pos: Vec2 },
    MovingAway { orig_pos: Vec3, transformer_pos: Vec2, transformer_spit_direction: Vec2 },
    Falling,
}

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
                    *square = SquarePos(square.1, w - square.0 - 1);
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
    spit_direction: Vec2,
}

impl Transformer {
    fn new(x: f32, y: f32, transformation: Transformation, spit_direction: Vec2) -> Self {
        Transformer {
            position: Vec2::new(x, y),
            radius: 35.0,
            transformation,
            spit_direction,
        }
    }
}


#[derive(Bundle)]
pub struct TransformerBundle {
    sprite_bundle: SpriteBundle,
    transformer: Transformer,
}

impl TransformerBundle {
    pub fn new(x: f32, y: f32, transformation: Transformation, spit_direction: Vec2, asset_server: &Res<AssetServer>) -> Self {
        let transformer = Transformer::new(x, y, transformation, spit_direction);
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_info: Query<(Entity, &mut Player, &mut Collider, &mut Path, &mut Transform)>,
    transformers: Query<&Transformer>,
) {
    for (player_entity, mut player, mut collider, mut path, player_transform) in &mut player_info {
        let mut collided_with_transformer = false;
        for transformer in &transformers {
            let distance = collider.distance_to_point(
                Vec2::new(player_transform.translation.x, player_transform.translation.y),
                0.0,
                transformer.position,
                true
            );
            if distance < transformer.radius {
                collided_with_transformer = true;
                match player.transformer_anim_state {
                    TransformerAnimState::NotAnimating => {
                        player.transformer_anim_state = TransformerAnimState::MovingToward {
                            orig_pos: player_transform.translation,
                            transformer_pos: transformer.position,
                        };
                        commands.entity(player_entity).insert(ColliderDisabled);
                        commands.spawn(AudioBundle {
                            source: asset_server.load("sounds/woosh_fast2.wav"),
                            ..default()
                        });
                    },
                    TransformerAnimState::MovingToward { orig_pos, transformer_pos } => {
                        if transformer_pos == transformer.position && distance < 0.01 {
                            player.transformer_anim_state = TransformerAnimState::MovingAway {
                                orig_pos,
                                transformer_pos: transformer.position,
                                transformer_spit_direction: transformer.spit_direction,
                            };
                            transformer.transformation.apply(&mut player);
                            *path = player.get_shape();
                            *collider = player.get_collider();
                        }
                    },
                    _ => {},
                }
            }
        }
        if !collided_with_transformer {
            if let TransformerAnimState::MovingAway { .. } = player.transformer_anim_state {
                commands.entity(player_entity).remove::<ColliderDisabled>();
                player.transformer_anim_state = TransformerAnimState::Falling;
            }
        }
    }
}
