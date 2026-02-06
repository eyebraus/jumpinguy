use bevy::state::state::States;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub(crate) enum ActionState {
    Dashing,
    #[default]
    Idle,
    Jumping,
    Walking,
}
