use bevy::{prelude::*, sprite::collide_aabb::Collision};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::collision::CollisionEvent;

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

fn wall_transform(kind: WallKind, mut translation: Vec2) -> SpriteBundle {
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

pub fn wall_collision_handling(
    mut transform_query: Query<&mut Transform>,
    query: Query<&Wall>,
    mut reader: EventReader<CollisionEvent>,
) {
    for event in reader.iter().filter(|event| query.get(event.other).is_ok()) {
        let mut transform = transform_query.get_mut(event.me).unwrap();
        match event.collision {
            Collision::Left => todo!(),
            Collision::Right => todo!(),
            Collision::Top => {
                transform.translation.y = (SCREEN_HEIGHT - transform.scale.y) / -2.;
            }
            Collision::Bottom => {
                transform.translation.y = (SCREEN_HEIGHT - transform.scale.y) / 2.;
            }
            _ => (),
        }
    }
}
