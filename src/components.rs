use bevy::ecs::component::Component;

#[derive(Component)]
pub struct InteractiveMode;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct PathLabel;

#[derive(Component, Debug)]
pub struct OrbitCamera {
    pub window_id: String,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub is_dragging: bool,
}
