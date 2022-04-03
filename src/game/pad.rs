use bevy::prelude::*;

use crate::util::default;

pub fn pad_bundle(x: f32, y: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: pad_sprite(),
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            ..default()
        },
        ..default()
    }
}

pub fn pad_sprite() -> Sprite {
    Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::new(20.0, 250.0)),
        ..default()
    }
}
