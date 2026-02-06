use bevy::ecs::component::Component;

#[derive(Clone, Component, Copy, Debug)]
pub(in crate::character) struct CharacterAsset;

#[derive(Clone, Component, Copy, Debug)]
pub(in crate::character) struct RogueSheetAsset;
