use bevy::{
    ecs::system::{Commands, Local, ResMut},
    render::view::screenshot::{save_to_disk, Screenshot},
};

use crate::resource::OperationWindowRelatedEntities;

// as the function name suggest, take a snapshot ( will be taking snapshot for the 3d model window )
pub fn take_snapshot(
    mut commands: Commands,
    mut counter: Local<u32>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
) {
    // TODO: refactor so that we path can be dynamic [set based on user]
    //       instead of hard code
    let path = format!("./screenshot/screenshot-{}.jpg", *counter);
    *counter += 1;

    // spawn the entity to capture snapshot of a window
    // NOTE: we need to tell the entity which window [Here it will be the window where the 3d model will be]
    commands
        .spawn(Screenshot::window(operation_window.window.unwrap()))
        .observe(save_to_disk(path));
}
