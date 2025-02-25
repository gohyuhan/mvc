use bevy::{
    ecs::system::{Commands, Local, ResMut},
    render::view::screenshot::{save_to_disk, Screenshot},
};

use crate::resource::OperationWindowRelatedEntities;

pub fn take_snapshot(
    mut commands: Commands,
    mut counter: Local<u32>,
    mut operationWindow: ResMut<OperationWindowRelatedEntities>,
) {
    let path = format!("./screenshot/screenshot-{}.jpg", *counter);
    *counter += 1;
    commands
        .spawn(Screenshot::window(operationWindow.window.unwrap()))
        .observe(save_to_disk(path));
}
