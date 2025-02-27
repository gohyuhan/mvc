use bevy::{
    asset::Handle,
    ecs::{entity::Entity, system::Resource},
    image::Image,
};

#[derive(Resource, Debug, Clone)]
pub struct AssetPath {
    pub model_path: String,
    pub skybox_path: String,
}

#[derive(Resource, Debug)]
pub struct SavePath {
    pub dir_path: String,
    pub dir_name: String,
    pub file_name_prefix: String,
}

#[derive(Resource)]
pub struct OperationWindowRelatedEntities {
    pub window: Option<Entity>,
    pub entities_list: Option<Vec<Entity>>,
}

#[derive(Resource)]
pub struct SkyboxAttribute {
    pub skybox_handler: Option<Handle<Image>>,
}

#[derive(Resource, Debug)]
pub struct LiveCameraPanNumber {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
}

#[derive(Resource, Debug)]
pub struct OperationSettings {
    pub yaw_min_value: f32,
    pub yaw_max_value: f32,
    pub pitch_min_value: f32,
    pub pitch_max_value: f32,
    pub radius_range: f32,
    pub radius_start_position: f32,
    pub model_rotate_sensitivity: f32,
    pub model_reposition_sensitivity: f32,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

#[derive(Resource, Debug)]
pub struct LiveCaptureOperationSettings {
    pub live_capture_iteration: usize,
    pub live_capture_iteration_current_counter: usize,
    pub live_capture_coordinate_list: Option<Vec<(f32, f32, f32)>>,
}
