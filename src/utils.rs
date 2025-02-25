use bevy::{
    prelude::*,
    state::{commands, state},
    window::{WindowCloseRequested, WindowCreated, WindowFocused},
};

use crate::{
    capture,
    resource::{ActiveWindowId, OperationWindowRelatedEntities},
    states::{AppState, IsCapture},
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
    mut capurestate: ResMut<NextState<IsCapture>>,
    mut operationWindow: ResMut<OperationWindowRelatedEntities>,
) {
    for ev in window_close_requested_events.read() {
        if ev.window == operationWindow.window.unwrap() {
            state.set(AppState::OperationEnd);
            capurestate.set(IsCapture::CaptureStop);
            for entity in operationWindow.entitiesList.as_mut().unwrap() {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}

// check keyboard in interactive mode for space bar to take snapshot
pub fn keyboard_interact(
    keys: Res<ButtonInput<KeyCode>>,
    mut captureState: ResMut<NextState<IsCapture>>,
    currentCaptureState: Res<State<IsCapture>>,
) {
    let s = currentCaptureState.as_ref().get();
    if *s == IsCapture::CaptureStop {
        if keys.just_pressed(KeyCode::Space) {
            println!("start capture");
            captureState.set(IsCapture::CaptureOngoing);
        }
    } else if *s == IsCapture::CaptureOngoing {
        if keys.just_pressed(KeyCode::Space) {
            println!("stop capture");
            captureState.set(IsCapture::CaptureStop);
        }
    }
}
