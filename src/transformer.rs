use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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

#[derive(Component)]
struct Transformer;

pub fn spawn_transformer(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 30.0,
        center: Vec2::new(-100.0, -175.0),
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        Fill::color(FILL_COLOR),
        Stroke::new(STROKE_COLOR, 10.0),
        Transformer,
    ));
}