use bevy::state::state::States;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub(in crate::character) enum LoadingState {
    #[default]
    Loading,
    Ready,
    Spawning,
}
