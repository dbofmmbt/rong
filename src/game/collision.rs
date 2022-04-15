use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[derive(Debug, Component)]
pub struct CollisionEvent {
    pub collision: Collision,
    pub me: Entity,
    pub other: Entity,
}

type Target<'a> = &'a Transform;

fn collided(a: Target, b: Target) -> Option<Collision> {
    collide(
        a.translation,
        a.scale.truncate(),
        b.translation,
        b.scale.truncate(),
    )
}

pub fn collision_detection(
    query: Query<(Entity, Target)>,
    mut writer: EventWriter<CollisionEvent>,
) {
    for (me, a) in query.iter() {
        trace!(?a);

        for (other, b) in query.iter().filter(|&(other, _)| me != other) {
            if let Some(collision) = collided(a, b) {
                writer.send(CollisionEvent {
                    collision,
                    other,
                    me,
                });
            }
        }
    }
}

pub fn collision_log(mut reader: EventReader<CollisionEvent>) {
    for event in reader.iter() {
        debug!(?event);
    }
}
