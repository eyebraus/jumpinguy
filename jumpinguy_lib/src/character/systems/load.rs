use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    image::Image,
    state::state::NextState,
};

use crate::{
    character::{
        components::character_asset::{CharacterAsset, RogueSheetAsset},
        values::loading_state::LoadingState,
    },
    sheet::{Sheet, SheetAsset},
};

pub(in crate::character) fn load_character_sheets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let configuration = asset_server.load("sheets/rogue/rogue.sheet.yaml");
    let image = asset_server.load("sheets/rogue/rogue.png");

    commands.spawn((
        CharacterAsset,
        RogueSheetAsset,
        Sheet::new(image, configuration),
    ));
}

pub(in crate::character) fn wait_for_character_sheets(
    character_sheets: Query<&Sheet, With<CharacterAsset>>,
    image_assets: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    sheet_assets: Res<Assets<SheetAsset>>,
) {
    let are_all_assets_loaded = character_sheets.iter().all(|sheet| {
        image_assets.get(sheet.image()).is_some()
            && sheet_assets.get(sheet.configuration()).is_some()
    });

    if are_all_assets_loaded {
        next_state.set(LoadingState::Spawning);
    }
}
