use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

const PLAYER_WIDTH: f32 = 50.0;
const PLAYER_HEIGHT: f32 = 50.0;


struct SquarePos(i32, i32);

#[derive(Component)]
pub struct Player {
    is_jumping: bool,
    squares: Vec<SquarePos>,
}

impl Player {
    fn new() -> Self {
        Player {
            is_jumping: false,
            squares: vec![SquarePos(0, 0), SquarePos(0, 1), SquarePos(1, 0)],
        }
    }

    fn get_shape(&self) -> Path {
        let mut builder = GeometryBuilder::new();
        for square in &self.squares {
            builder = builder.add(&shapes::Rectangle {
                extents: Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
                origin: RectangleOrigin::CustomCenter(Vec2::new(PLAYER_WIDTH * square.0 as f32, PLAYER_HEIGHT * square.1 as f32)),
            });
        }
        builder.build()
    }

    fn get_collider(&self) -> Collider {
        let shape_tuples = self.squares.iter().map(|square|
            (
                Vec2::new(PLAYER_WIDTH * square.0 as f32, PLAYER_HEIGHT * square.1 as f32),
                0.0,
                Collider::cuboid(PLAYER_WIDTH/2.0, PLAYER_HEIGHT/2.0),
            )
        ).collect();

        Collider::compound(shape_tuples)
    }
}


pub fn spawn_player(mut commands: Commands) {
    let player = Player::new();
    commands.spawn((
        ShapeBundle {
            path: player.get_shape(),
            ..default()
        },
        Fill::color(Color::YELLOW),
        RigidBody::Dynamic,
        player.get_collider(),
        ActiveEvents::CONTACT_FORCE_EVENTS,
        Sleeping::disabled(),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(2.0),
        GravityScale(4.0),
        Velocity::zero(),
        player,
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
    mut contact_events: EventReader<ContactForceEvent>,
    mut players: Query<(Entity, &mut Player)>,
) {
    for (entity, mut player) in &mut players {
        for contact_event in contact_events.iter() {
            if (contact_event.collider1 == entity || contact_event.collider2 == entity)
                    && contact_event.total_force.y != 0.0 {
                player.is_jumping = false;
            }
        }
    }
}
