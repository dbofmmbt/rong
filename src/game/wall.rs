use bevy::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::util::default;

#[derive(Debug, Component)]
pub struct Wall;

pub const WALL_SIZE: f32 = 100.;

pub fn create_window_boundaries(commands: &mut Commands) {
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Vertical,
        Vec2::new((SCREEN_WIDTH + WALL_SIZE) / -2., 0.),
    ));
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Vertical,
        Vec2::new((SCREEN_WIDTH + WALL_SIZE) / 2., 0.),
    ));
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Horizontal,
        Vec2::new(0., (SCREEN_HEIGHT + WALL_SIZE) / 2.),
    ));
    commands.spawn().insert(Wall).insert_bundle(wall_transform(
        WallKind::Horizontal,
        Vec2::new(0., (SCREEN_HEIGHT + WALL_SIZE) / -2.),
    ));
}

enum WallKind {
    Vertical,
    Horizontal,
}

fn wall_transform(kind: WallKind, translation: Vec2) -> SpriteBundle {
    let (width, height) = match kind {
        WallKind::Vertical => (WALL_SIZE, SCREEN_HEIGHT),
        WallKind::Horizontal => (SCREEN_WIDTH - WALL_SIZE, WALL_SIZE),
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
