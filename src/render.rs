use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::RenderTarget,
    window::{WindowRef, WindowResolution},
};

use crate::{components::OrbitCamera, resource::OperationWindowRelatedEntities};

pub fn interactive(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    path: String,
    mut operationWindow: ResMut<OperationWindowRelatedEntities>,
) {
    let interac_window = commands
        .spawn(Window {
            title: "Interactive Mode".to_owned(),
            resolution: WindowResolution::new(1500., 1000.),
            ..default()
        })
        .id();

    println!("interac_window: {:?}", interac_window);

    let interac_window_camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 1.0, 2.5).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            EnvironmentMapLight {
                diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
                intensity: 250.0,
                ..default()
            },
            Camera {
                target: RenderTarget::Window(WindowRef::Entity(interac_window)),
                ..default()
            },
        ))
        .insert(OrbitCamera {
            window_id: interac_window.to_string(),
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

    let node_entiry = commands
        .spawn((node, TargetCamera(interac_window_camera)))
        .id();

    let entities_list: Vec<Entity> = vec![
        interac_window,
        interac_window_camera,
        directional_light,
        scene_entity,
        node_entiry,
    ];
    operationWindow.window = Some(interac_window);
    operationWindow.entitiesList = Some(entities_list)
}

// set the ambient light that is used for the scene
pub fn setup_ambient_light(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 300.0;
}

pub fn orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    for q in query.iter_mut() {
        let (mut transform, mut orbit) = q;

        // Handle left mouse button for drag
        if buttons.just_pressed(MouseButton::Left) {
            orbit.is_dragging = true;
        }
        if buttons.just_released(MouseButton::Left) {
            orbit.is_dragging = false;
        }

        // Orbiting when dragging
        if orbit.is_dragging {
            for ev in motion_evr.read() {
                let sensitivity = 0.01;
                orbit.yaw -= ev.delta.x * sensitivity;
                orbit.pitch += ev.delta.y * sensitivity;

                // Clamp pitch to avoid flipping
                orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);
            }
        }

        // Zoom with scroll wheel
        for ev in scroll_evr.read() {
            orbit.radius -= ev.y * 0.5;
            orbit.radius = orbit.radius.clamp(2.0, 50.0);
        }

        // Calculate new camera position
        let yaw_rot = Quat::from_rotation_y(orbit.yaw);
        let pitch_rot = Quat::from_rotation_x(orbit.pitch);
        let offset = yaw_rot * pitch_rot * Vec3::new(0.0, 0.0, orbit.radius);

        transform.translation = Vec3::ZERO + offset;
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
