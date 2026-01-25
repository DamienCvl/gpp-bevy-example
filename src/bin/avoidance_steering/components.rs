use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec2,
};

#[derive(Component)]
pub struct Mover {
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub target: Option<Entity>,
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Obstacle {
    pub radius: f32,
}
