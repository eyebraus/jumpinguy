use bevy::{
    ecs::system::{Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    state::state::{NextState, State},
};

use crate::state::values::game_state::GameState;

pub(in crate::state) fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Paused => {
                next_state.set(GameState::Playing);
            }

            GameState::Playing => {
                next_state.set(GameState::Paused);
            }

            _ => {}
        }
    }
}
