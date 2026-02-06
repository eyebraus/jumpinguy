use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{IntoScheduleConfigs, SystemCondition},
    state::{app::AppExtStates, condition::in_state},
};

use crate::state::systems::pause::toggle_pause;
use crate::state::values::game_state::GameState;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            toggle_pause.run_if(in_state(GameState::Paused).or(in_state(GameState::Playing))),
        );
    }
}
