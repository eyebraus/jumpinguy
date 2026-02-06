use bevy::state::state::States;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub(crate) enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
}
