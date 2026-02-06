use anyhow::{Context, anyhow};
use bevy::{
    asset::Assets,
    ecs::{
        entity::{ContainsEntity, Entity},
        query::With,
        system::{Commands, Query, Res, ResMut, Single},
    },
    image::{Image, TextureAtlas, TextureAtlasLayout},
    math::{UVec2, Vec3},
    sprite::Sprite,
    state::state::NextState,
    transform::components::Transform,
};

use crate::{
    character::{CharacterAsset, LoadingState, RogueSheetAsset},
    sheet::{Sheet, SheetAsset},
};

pub(in crate::character) fn spawn_character(
    mut commands: Commands,
    image_assets: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<LoadingState>>,
    rogue_sheet: Single<&Sheet, With<RogueSheetAsset>>,
    sheet_assets: Res<Assets<SheetAsset>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> anyhow::Result<()> {
    // Get assets loaded for the character's spritesheet
    let image = rogue_sheet.image().clone();

    let sheet = sheet_assets
        .get(rogue_sheet.configuration())
        .ok_or(anyhow!("Character sheet configuration is not yet loaded."))?;

    // Create texture atlas layout
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        sheet.tile_size(),
        sheet.columns() as u32,
        sheet.rows() as u32,
        None,
        None,
    ));

    // Spawn player entity
    commands.spawn((
        Sprite::from_atlas_image(image, layout.into()),
        Transform::from_scale(Vec3::splat(4.0)),
    ));

    Ok(())
}
