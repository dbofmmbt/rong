use crate::util::default;
use bevy::prelude::*;

pub fn ball_bundle(x: f32, y: f32) -> impl Bundle {
    const SIZE: f32 = 50.;

    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(SIZE, SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(x, y, 0.),
        ..default()
    }
}
