use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const PLAYER_WIDTH: f32 = 50.0;
pub const PLAYER_HEIGHT: f32 = 50.0;

#[derive(Copy, Clone)]
pub struct SquarePos(pub i32, pub i32);

#[derive(Component)]
pub struct Player {
    is_jumping: bool,
    pub is_being_transformed: bool,
    squares: Vec<SquarePos>,
}

impl Player {
    fn new() -> Self {
        Player {
            is_jumping: false,
            is_being_transformed: false,
            squares: vec![SquarePos(0, 0)],
        }
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
        self.squares.push(SquarePos(x, y));
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
            300.0
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
