use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_map(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(500.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
        Collider::cuboid(500.0/2.0, 50.0/2.0),
    ));
}
