use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct InteractiveMode;

#[derive(Component)]
pub struct PathLabel;

#[derive(Component, Debug)]
pub struct OrbitCamera {
    pub window: Entity,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub is_dragging: bool,
}
