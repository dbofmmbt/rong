use bevy::prelude::*;
use heron::{CollisionShape, PhysicMaterial, RigidBody, Velocity};

use crate::util::default;

#[derive(Bundle)]
struct BallBundle {
    #[bundle]
    sprite: SpriteBundle,
    velocity: Velocity,
    collision_shape: CollisionShape,
    rigid_body: RigidBody,
    material: PhysicMaterial,
}

pub fn ball_bundle(x: f32, y: f32) -> impl Bundle {
    const SIDE_SIZE: f32 = 50.;
    let size = Vec3::new(SIDE_SIZE, SIDE_SIZE, SIDE_SIZE);

    BallBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: size,
                ..default()
            },
            ..default()
        },
        velocity: Velocity::from(Vec3::new(500., 1000., 0.)),
        collision_shape: CollisionShape::Sphere {
            radius: SIDE_SIZE / 2.,
        },
        rigid_body: RigidBody::Dynamic,
        material: PhysicMaterial {
            restitution: 1.,
            density: 1.,
            friction: 0.,
        },
    }
}
