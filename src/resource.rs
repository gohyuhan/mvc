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
    pub path: String,
}

#[derive(Resource)]
pub struct ActiveWindowId {
    pub id: String,
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
