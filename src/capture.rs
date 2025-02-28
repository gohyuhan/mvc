use std::path::Path;

use bevy::{
    ecs::system::{Commands, Res},
    render::view::screenshot::{save_to_disk, Screenshot},
};

use crate::resource::{OperationWindowRelatedEntities, SavePath};

// as the function name suggest, take a snapshot ( will be taking snapshot for the 3d model window )
pub fn take_snapshot(
    mut commands: Commands,
    operation_window: Res<OperationWindowRelatedEntities>,
    save_settings: &SavePath,
    yaw: f32,
    pitch: f32,
    radius: f32,
) {
    let path = Path::new(&save_settings.current_dir_path).join(format!(
        "{}_{}_{}_{}.jpg",
        save_settings.file_name_prefix, yaw, pitch, radius
    ));

    // spawn the entity to capture snapshot of a window
    // NOTE: we need to tell the entity which window [Here it will be the window where the 3d model will be]
    commands
        .spawn(Screenshot::window(
            operation_window.window.expect("window not found"),
        ))
        .observe(save_to_disk(path));
}
