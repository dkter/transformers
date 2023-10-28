use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_WIDTH: f32 = 50.0;
const PLAYER_HEIGHT: f32 = 50.0;

#[derive(Component)]
pub struct Player {
    is_jumping: bool,
}

impl Player {
    fn new() -> Self {
        Player {
            is_jumping: false,
        }
    }
}


pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(PLAYER_WIDTH/2.0, PLAYER_HEIGHT/2.0),
        ActiveEvents::CONTACT_FORCE_EVENTS,
        Sleeping::disabled(),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(2.0),
        GravityScale(4.0),
        Velocity::zero(),
        Player::new(),
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&mut Player, &mut Velocity)>,
) {
    for (mut player, mut velocity) in &mut player_info {
        let left = keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::Right);

        let x = (-(left as i8) + right as i8) as f32;
        let y_delta = if keyboard_input.pressed(KeyCode::Up) && !player.is_jumping {
            player.is_jumping = true;
            200.0
        } else {
            0.0
        };
        let y = velocity.linvel.y + y_delta;

        velocity.linvel = Vec2::new(x * 200.0, y);
    }
}

pub fn set_jumping_false(
    mut collision_events: EventReader<ContactForceEvent>,
    mut players: Query<(Entity, &mut Player)>,
) {
    for (entity, mut player) in &mut players {
        for contact_event in collision_events.iter() {
            if contact_event.collider1 == entity || contact_event.collider2 == entity {
                player.is_jumping = false;
            }
        }
    }
}
