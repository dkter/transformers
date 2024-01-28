use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::{
    LevelTransitioning,
    transformer::TransformerAnimState,
};

pub const PLAYER_WIDTH: f32 = 50.0;
pub const PLAYER_HEIGHT: f32 = 50.0;

const PLAYER_COLOR: Color = Color::Rgba {
    red: 0.96484375,
    green: 0.828125,
    blue: 0.1796875,
    alpha: 1.0,
};

#[derive(Copy, Clone)]
pub struct SquarePos(pub i32, pub i32);

#[derive(Component)]
pub struct Player {
    is_jumping: bool,
    pub transformer_anim_state: TransformerAnimState,
    pub squares: Vec<SquarePos>,
}

impl Player {
    fn new() -> Self {
        Player {
            is_jumping: false,
            transformer_anim_state: TransformerAnimState::NotAnimating,
            squares: vec![SquarePos(0, 0)],
        }
    }
    
    pub fn get_dimens(&self) -> (i32, i32) {
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

    pub fn get_collider(&self) -> Collider {
        let shape_tuples = self.squares.iter().map(|square|
            (
                Vec2::new(PLAYER_WIDTH * square.0 as f32, PLAYER_HEIGHT * square.1 as f32),
                0.0,
                Collider::cuboid(PLAYER_WIDTH/2.0, PLAYER_HEIGHT/2.0),
            )
        ).collect();

        Collider::compound(shape_tuples)
    }

    pub fn add_square(&mut self, x: i32, y: i32) {
        for square in &self.squares {
            if square.0 == x && square.1 == y {
                return;
            }
        }
        self.squares.push(SquarePos(x, y));
    }
}


pub fn spawn_player_at_point(commands: &mut Commands, spawn_point: (f32, f32)) {
    let player = Player::new();
    commands.spawn((
        ShapeBundle {
            path: player.get_shape(),
            transform: Transform::from_xyz(spawn_point.0, spawn_point.1, 0.0),
            ..default()
        },
        Fill::color(PLAYER_COLOR),
        RigidBody::Dynamic,
        player.get_collider(),
        ActiveEvents::CONTACT_FORCE_EVENTS,
        Sleeping::disabled(),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(2.0),
        GravityScale(5.0),
        Velocity::zero(),
        player,
    ));
}


pub fn spawn_player(mut commands: Commands) {
    spawn_player_at_point(&mut commands, (-550.0, -200.0));  // the spawn point for the first level
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&mut Player, &mut Velocity, &mut Transform)>,
    level_transitioning: ResMut<LevelTransitioning>,
) {
    if level_transitioning.0 {
        return;
    }
    for (mut player, mut velocity, mut transform) in &mut player_info {
        match player.transformer_anim_state {
            TransformerAnimState::NotAnimating => {
                // move normally
                let left = keyboard_input.pressed(KeyCode::Left);
                let right = keyboard_input.pressed(KeyCode::Right);

                let x = 300.0 * (-(left as i8) + right as i8) as f32;
                let y_delta = if keyboard_input.pressed(KeyCode::Up) && !player.is_jumping {
                    player.is_jumping = true;
                    350.0
                } else {
                    0.0
                };
                let y = velocity.linvel.y + y_delta;

                velocity.linvel = Vec2::new(x, y);
            },
            TransformerAnimState::MovingToward { orig_pos, transformer_pos } => {
                // move toward transformer
                velocity.linvel = Vec2::new(
                    transformer_pos.x - orig_pos.x,
                    transformer_pos.y - orig_pos.y,
                ) * 5.0;
                transform.scale *= 0.9;
            },
            TransformerAnimState::MovingAway { orig_pos, transformer_pos, transformer_spit_direction } => {
                // move away from transformer
                velocity.linvel = Vec2::new(
                    transformer_spit_direction.x * (transformer_pos.x - orig_pos.x).signum(),
                    transformer_spit_direction.y,
                );
                if transform.scale.x < 1.0 {
                    transform.scale *= 1.1;
                }
                if transform.scale.x > 1.0 {
                    transform.scale = Vec3::new(1.0, 1.0, 1.0);
                }
            },
            TransformerAnimState::Falling => {
                if transform.scale.x < 1.0 {
                    transform.scale *= 1.1;
                }
                if transform.scale.x > 1.0 {
                    transform.scale = Vec3::new(1.0, 1.0, 1.0);
                }
            },
        }
    }
}

pub fn set_jumping_false(
    mut contact_events: EventReader<ContactForceEvent>,
    mut players: Query<(Entity, &mut Player)>,
) {
    for (entity, mut player) in &mut players {
        for contact_event in contact_events.iter() {
            if (contact_event.collider1 == entity || contact_event.collider2 == entity)
                    && contact_event.total_force.y != 0.0 {
                player.is_jumping = false;
                if let TransformerAnimState::Falling {..} = player.transformer_anim_state {
                    player.transformer_anim_state = TransformerAnimState::NotAnimating;
                }
            }
        }
    }
}
