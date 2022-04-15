use bevy::prelude::*;
use heron::{CollisionShape, PhysicMaterial, RigidBody, RotationConstraints};

use crate::util::default;

#[derive(Bundle, Default)]
struct PadBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    physic_material: PhysicMaterial,
    shape: CollisionShape,
    rotation: RotationConstraints,
}

pub fn pad_bundle(x: f32, y: f32) -> impl Bundle {
    let size = Vec2::new(20.0, 250.0);

    PadBundle {
        sprite_bundle: SpriteBundle {
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
        },
        rigid_body: RigidBody::Dynamic,
        physic_material: PhysicMaterial {
            restitution: 1.,
            density: 1_000_000.,
            friction: 0.,
        },
        shape: CollisionShape::Cuboid {
            half_extends: size.extend(1.) / 2.,
            border_radius: None,
        },
        rotation: RotationConstraints::lock(),
    }
}
