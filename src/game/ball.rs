use bevy::prelude::*;

use crate::util::default;

pub fn ball_bundle(x: f32, y: f32) -> impl Bundle {
    const SIDE_SIZE: f32 = 50.;
    let size = Vec2::new(SIDE_SIZE, SIDE_SIZE);
    SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(x, y, 0.),
            scale: size.extend(1.),
            ..default()
        },
        ..default()
    }
}
