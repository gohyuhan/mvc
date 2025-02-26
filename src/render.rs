use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{WindowRef, WindowResolution},
};

use crate::{components::OrbitCamera, resource::OperationWindowRelatedEntities};

// this will be the function responsible to spawn a window for the 3d model to render in
pub fn interactive(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    path: String,
    mut operation_window: ResMut<OperationWindowRelatedEntities>,
) {
    // spawn a new window ( In MVC, there will be a maximum of 2 window at the same time, 1 for MVC main menu and the other will be for 3d model )
    let interac_window = commands
        .spawn(Window {
            title: "Interactive Mode".to_owned(),
            resolution: WindowResolution::new(1500., 1000.),
            ..default()
        })
        .id();

    // Spawn the camera for the 3d model window
    let interac_window_camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 1.0, 2.5).looking_at(Vec3::new(0.0, 0.0, 100.0), Vec3::Y),
            EnvironmentMapLight {
                diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
                intensity: 250.0,
                ..default()
            },
            // IMPORTANT, we need to tell the camera where to target
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(interac_window)),
                hdr: true,
                ..default()
            },
        ))
        // this will be relavent for use to control the orbiting of the model
        .insert(OrbitCamera {
            window: interac_window,
            radius: 2.5,
            yaw: 1.0,
            pitch: 0.0,
            is_dragging: false,
        })
        .id();

    // directional light
    let directional_light = commands
        .spawn((DirectionalLight {
            illuminance: 300.0,
            shadows_enabled: false,
            ..default()
        },))
        .id();

    // spawn the 3d model
    let scene_entity = commands
        .spawn((SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone())),
        ),))
        .id();

    let node = Node {
        position_type: PositionType::Absolute,
        top: Val::Px(12.0),
        left: Val::Px(12.0),
        ..default()
    };

    let node_entity = commands
        .spawn((node, TargetCamera(interac_window_camera)))
        .id();

    let entities_list: Vec<Entity> = vec![
        interac_window,
        interac_window_camera,
        directional_light,
        scene_entity,
        node_entity,
    ];

    // saving the entites to a list, so that we can easily despawn them when the window close
    operation_window.window = Some(interac_window);
    operation_window.entities_list = Some(entities_list)
}

// set the ambient light that is used for the scene
pub fn setup_ambient_light(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 300.0;
}
