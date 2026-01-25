use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    math::{Vec2, Vec3, Vec3Swizzles},
    time::Time,
    transform::components::Transform,
};

use crate::{
    components::{Mover, Target},
    constants::{ENTITY_ACCELERATION_TIME_TOWARD_TARGET, ENTITY_SPEED},
};

pub fn apply_movement_system(mut query: Query<(&mut Transform, &mut Mover)>, time: Res<Time>) {
    for (mut transform, mut mover) in query.iter_mut() {
        mover.velocity = (mover.velocity + mover.acceleration * time.delta_secs())
            .clamp_length_max(ENTITY_SPEED);

        mover.acceleration = Vec2::ZERO;

        if mover.velocity.length() < f32::EPSILON {
            mover.velocity = Vec2::ZERO;
        }

        transform.translation += mover.velocity.extend(0.0) * time.delta_secs();

        if mover.velocity.length_squared() > 0.0 {
            set_2d_rotation(&mut transform, &mover.velocity);
        }
    }
}

pub fn goto_target_system(
    mut mover_query: Query<(&Transform, &mut Mover)>,
    target_query: Query<&Transform, With<Target>>,
) {
    for (mover_transform, mut mover) in mover_query.iter_mut() {
        let desired = mover
            .target
            .and_then(|target| target_query.get(target).ok())
            .map(|t| {
                (t.translation.xy() - mover_transform.translation.xy()).normalize_or_zero()
                    * ENTITY_SPEED
            })
            .unwrap_or(Vec2::ZERO);

        mover.acceleration += desired / ENTITY_ACCELERATION_TIME_TOWARD_TARGET;
    }
}

fn set_2d_rotation(transform: &mut Transform, velocity: &Vec2) {
    let forward = (transform.rotation * Vec3::Y).xy();
    let to = velocity.normalize();
    let angle = forward.angle_to(to);
    transform.rotate_z(angle);
}
