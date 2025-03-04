use bevy::state::state::States;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    OperationMode,
    ModelSwitchingMode,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum OperationState {
    #[default]
    None,
    Interactive,
    LiveCapture,
    LivePreview,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum CameraFovInitializedState {
    #[default]
    NotInitialized,
    Initialized,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum RenderModelForwardOrBackward {
    #[default]
    Forward,
    Backward,
}
