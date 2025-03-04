use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub image_save_dir: String,
    pub yaw_min_value: f32,
    pub yaw_max_value: f32,
    pub pitch_min_value: f32,
    pub pitch_max_value: f32,
    pub radius_range: f32,
    pub model_rotate_sensitivity: f32,
    pub model_reposition_sensitivity: f32,
    pub mouse_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub live_capture_iteration: usize,
}

#[derive(Debug, Clone)]
pub struct SavePath {
    pub current_dir_path: String,
    pub file_name_prefix: String,
}
