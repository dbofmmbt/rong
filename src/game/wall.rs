use bevy::prelude::*;
use heron::{CollisionShape, PhysicMaterial, RigidBody};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::util::default;

#[derive(Debug, Component)]
pub struct Wall;

pub const WALL_SIZE: f32 = 100.;

use WallKind::*;

pub fn create_window_boundaries(commands: &mut Commands) {
    let horizontal_position = (SCREEN_WIDTH + WALL_SIZE) / 2.;
    let vertical_position = (SCREEN_HEIGHT + WALL_SIZE) / 2.;

    create_wall(commands, Vertical, Vec2::new(horizontal_position, 0.));
    create_wall(commands, Vertical, Vec2::new(-horizontal_position, 0.));

    create_wall(commands, Horizontal, Vec2::new(0., vertical_position));
    create_wall(commands, Horizontal, Vec2::new(0., -vertical_position));
}

fn create_wall(commands: &mut Commands, kind: WallKind, translation: Vec2) {
    let bundle = wall_sprite_bundle(kind, translation);
    commands
        .spawn()
        .insert(Wall)
        .insert(shape(bundle.transform.scale))
        .insert_bundle(bundle)
        .insert(RigidBody::Static)
        .insert(PhysicMaterial {
            restitution: 1.,
            density: 1.,
            friction: 0.,
        });
}

fn shape(scale: Vec3) -> CollisionShape {
    CollisionShape::Cuboid {
        half_extends: scale / 2.,
        border_radius: None,
    }
}

enum WallKind {
    Vertical,
    Horizontal,
}

fn wall_sprite_bundle(kind: WallKind, translation: Vec2) -> SpriteBundle {
    let (width, height) = match kind {
        WallKind::Vertical => (WALL_SIZE, SCREEN_HEIGHT),
        WallKind::Horizontal => (SCREEN_WIDTH, WALL_SIZE),
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
