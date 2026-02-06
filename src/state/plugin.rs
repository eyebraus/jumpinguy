use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{IntoScheduleConfigs, SystemCondition},
    state::{app::AppExtStates, condition::in_state},
};

use crate::state::systems::toggle_pause;
use crate::state::values::GameState;

pub(crate) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            toggle_pause.run_if(in_state(GameState::Paused).or(in_state(GameState::Playing))),
        );
    }
}
