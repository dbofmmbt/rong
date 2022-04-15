use bevy::prelude::*;

use crate::util::default;

pub fn pad_bundle(x: f32, y: f32) -> SpriteBundle {
    let size = Vec2::new(20.0, 250.0);
    SpriteBundle {
        sprite: pad_sprite(),
        transform: Transform {
            translation: Vec3::new(x, y, 0.),
            scale: size.extend(1.),
            ..default()
        },
        ..default()
    }
}

pub fn pad_sprite() -> Sprite {
    Sprite {
        color: Color::WHITE,
        ..default()
    }
}
