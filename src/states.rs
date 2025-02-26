use bevy::state::state::States;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    OperationMode,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum OperationState {
    #[default]
    None,
    Interactive,
    LiveCapture,
    LivePreview,
}
