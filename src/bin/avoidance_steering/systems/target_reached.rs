use bevy::{
    ecs::{query::With, system::Query},
    math::Vec3Swizzles,
    transform::components::Transform,
};

use crate::{
    components::{Mover, Target},
    constants::{ENTITY_DESELERATION_TIME_AFTER_TARGET_REACHED, TARGET_REACHED_DISTANCE},
};

pub fn target_reached_system(
    mut mover_query: Query<(&Transform, &mut Mover)>,
    target_query: Query<&Transform, With<Target>>,
) {
    for (mover_transform, mut mover) in mover_query.iter_mut() {
        if let Some(target_entity) = mover.target
            && let Ok(target_transform) = target_query.get(target_entity)
        {
            let distance = mover_transform
                .translation
                .xy()
                .distance(target_transform.translation.xy());

            if distance < TARGET_REACHED_DISTANCE {
                mover.target = None;
            }
        }

        if mover.target.is_none() && mover.velocity.length_squared() > 0.0 {
            let deseleration = -mover.velocity / ENTITY_DESELERATION_TIME_AFTER_TARGET_REACHED;
            mover.acceleration += deseleration;
        }
    }
}
