use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{app::AppExtStates, condition::in_state},
};

use crate::character::{
    ActionState, LoadingState, load_character_sheets, wait_for_character_sheets,
};

pub(crate) struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<ActionState>()
            .init_state::<LoadingState>()
            .add_systems(Startup, load_character_sheets)
            .add_systems(
                Update,
                wait_for_character_sheets.run_if(in_state(LoadingState::Loading)),
            );
    }
}
