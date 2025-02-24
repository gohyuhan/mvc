use bevy::state::state::States;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    OperationEnd, // model rendering operation end or not started
    OperationStart, // model rendering operation start or ongoing
}
