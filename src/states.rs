use bevy::state::state::States;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    OperationEnd, // model rendering operation end or not started
    OperationStart, // model rendering operation start or ongoing
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum IsCapture {
    #[default]
    CaptureOngoing, // should capture
    CaptureStop, // shoud not capture
}
