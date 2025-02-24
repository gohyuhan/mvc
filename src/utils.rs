use bevy::{
    prelude::*,
    state::{commands, state},
    window::{WindowCloseRequested, WindowCreated, WindowFocused},
};

use crate::{
    resource::{ActiveWindowId, OperationWindowRelatedEntities},
    states::AppState,
};

pub fn track_active_window(
    mut events: EventReader<WindowFocused>,
    mut activeWindowId: ResMut<ActiveWindowId>,
) {
    for event in events.read() {
        if event.focused {
            println!("Active window updated: {:?}", event.window);
            activeWindowId.id = format!("{:?}", event.window);
        }
    }
}

pub fn switch_state_on_window_event(
    mut commands: Commands,
    windows: Query<Entity, With<Window>>,
    entities: Query<Entity>,
    mut window_close_requested_events: EventReader<WindowCloseRequested>,
    mut state: ResMut<NextState<AppState>>,
    mut operationWindow: ResMut<OperationWindowRelatedEntities>,
) {
    for ev in window_close_requested_events.read() {
        println!("Window Close Requested - Switching back to Main Menu");

        // Only switch state if not already in Main Menu
        println!("Current State: {:?}", ev.window);

        println!("windows: {:?}", windows);
        println!("entities: {:?}", entities);
        if ev.window == operationWindow.window.unwrap() {
            state.set(AppState::OperationEnd);
            for entity in operationWindow.entitiesList.as_mut().unwrap() {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}
