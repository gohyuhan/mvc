use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::window::WindowTheme;

mod menu;
use menu::*;

mod components;

mod resource;
use resource::*;

mod render;
use render::*;

mod states;
use states::*;

mod utils;
use utils::*;

mod capture;

mod camera;
use camera::*;

// Note:
// The keyboard command when in the interactive mode
//
// C: capture 1 snapshot
// L: enter or exit live preview mode
// I: enter or exit interactive mode
// space: enter or exit live capture mode
// arrow key: move the model [ soon will be change to rotate model ]
// wasd: move the camera
// mouse wheel: zoom in or out
// mouse left click and move: rotate the model


fn main() {
    let mut app = App::new();
    // set the resource
    app.insert_resource(DirectionalLightShadowMap { size: 4096 });
    app.insert_resource(AssetPath {
        model_path: "".to_string(),
        skybox_path: "".to_string(),
    });
    app.insert_resource(SavePath {
        path: "".to_string(),
    });
    app.insert_resource(ActiveWindowId {
        id: "temp".to_string(),
    });
    app.insert_resource(SkyboxAttribute {
        skybox_handler: None,
    });
    app.insert_resource(OperationWindowRelatedEntities {
        window: None,
        entities_list: None,
    });
    app.insert_resource(LiveCameraPanNumber {
        yaw: 1.0,
        pitch: 1.0,
        radius: 1.0,
    });
    // set the plugins
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "MVC MAIN MENU!".into(),
            name: Some("MVC".into()),
            position: WindowPosition::At(IVec2 { x: 1, y: 1 }),
            resolution: (500., 750.).into(),
            // Tells Wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            // This will spawn an invisible window
            // The window will be made visible in the make_visible() system after 3 frames.
            // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
            visible: true,
            ..default()
        }),
        ..default()
    }),));
    // set initial state
    app.insert_state(AppState::MainMenu);
    app.insert_state(OperationState::None);
    app.add_systems(Startup, menu);
    app.add_systems(
        Update,
        (
            file_drag_and_drop_system,
            button_click_system.run_if(in_state(AppState::MainMenu)),
            keyboard_interact.run_if(in_state(AppState::OperationMode)),
            reposition_model.run_if(in_state(OperationState::Interactive)),
            setup_ambient_light,
            track_active_window,
            interactive_orbit_camera.run_if(in_state(AppState::OperationMode)),
            live_orbit_camera.run_if(in_state(AppState::OperationMode)),
            switch_state_on_window_event,
        ),
    );

    app.run();
}
