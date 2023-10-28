use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

struct Block {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Block {
    fn to_sprite_bundle(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(self.w, self.h)),
                ..default()
            },
            transform: Transform::from_xyz(self.x, self.y, 0.0),
            ..default()
        }
    }

    fn to_collider(&self) -> Collider {
        Collider::cuboid(self.w / 2.0, self.h / 2.0)
    }
}

const LEVEL0: [Block; 2] = [
    Block { x: 0.0, y: -200.0, w: 500.0, h: 50.0 },
    Block { x: 200.0, y: -200.0, w: 50.0, h: 150.0 },
];

pub fn spawn_map(mut commands: Commands) {
    for block in LEVEL0 {
        commands.spawn((
            block.to_sprite_bundle(),
            block.to_collider(),
        ));
    }
}
