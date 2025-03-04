use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct InteractiveMode;

#[derive(Component)]
pub struct ClearModelAssetsButton;

#[derive(Component)]
pub struct ClearSkyboxAssetsButton;

#[derive(Component)]
pub struct ModelPathLabel;

#[derive(Component)]
pub struct SkyboxPathLabel;

#[derive(Component, Debug)]
pub struct OrbitCamera {
    pub window: Entity,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub is_dragging: bool,
}

#[derive(Component, Debug)]
pub struct ModelRotateReposition {
    pub window: Entity,
    pub x: f32,
    pub y: f32,
}
