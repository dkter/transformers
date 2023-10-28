use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::player::Player;

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

enum Transformation {
    AddRight,
}

impl Transformation {
    fn apply(&mut self, player: &mut Player) {
        match self {
            Transformation::AddRight => {
                // in the future, figure out what the bottom right pos is, but for now
                player.add_square(1, 0);
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


pub fn spawn_transformer(mut commands: Commands) {
    let transformer = Transformer::new(-100.0, -175.0, Transformation::AddRight);
    let shape = transformer.get_shape();
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        Fill::color(FILL_COLOR),
        Stroke::new(STROKE_COLOR, 10.0),
        transformer,
    ));
}
