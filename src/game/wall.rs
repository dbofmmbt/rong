use bevy::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug, Component)]
struct Wall;

pub fn create_window_boundaries(commands: &mut Commands) {
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Vertical,
        Vec2::new(SCREEN_WIDTH / -2., 0.),
    ));
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Vertical,
        Vec2::new(SCREEN_WIDTH / 2., 0.),
    ));
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Horizontal,
        Vec2::new(0., SCREEN_HEIGHT / 2.),
    ));
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Horizontal,
        Vec2::new(0., SCREEN_HEIGHT / -2.),
    ));
}

enum WallKind {
    Vertical,
    Horizontal,
}

fn wall_transform(kind: WallKind, mut translation: Vec2) -> SpriteBundle {
    let wall_size = 100.;
    let (width, height) = match kind {
        WallKind::Vertical => {
            if translation.x.is_sign_positive() {
                translation.x += wall_size;
            } else {
                translation.x -= wall_size;
            }
            (wall_size, SCREEN_HEIGHT)
        }
        WallKind::Horizontal => {
            if translation.y.is_sign_positive() {
                translation.y += wall_size;
            } else {
                translation.y -= wall_size;
            }
            (SCREEN_WIDTH, wall_size)
        }
    };
    SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            translation: translation.extend(0.),
            scale: Vec3::new(width, height, 1.),
            ..default()
        },
        ..default()
    }
}
