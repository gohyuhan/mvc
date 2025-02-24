use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Resource, Debug)]
pub struct AssetPath {
    pub path: String,
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
    pub entitiesList: Option<Vec<Entity>>,
}
