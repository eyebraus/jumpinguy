use bevy::{asset::Asset, math::UVec2, reflect::TypePath};
use serde::Deserialize;

use crate::sheet::{Grid, Strip, Tile};

#[derive(Asset, Clone, Debug, Deserialize, TypePath)]
pub(crate) struct SheetAsset {
    grid: Grid,
    name: String,
    strips: Vec<Strip>,
    tile: Tile,
}

impl SheetAsset {
    pub fn columns(&self) -> u8 {
        self.grid.columns()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rows(&self) -> u8 {
        self.grid.rows()
    }

    pub fn strips(&self) -> &Vec<Strip> {
        &self.strips
    }

    pub fn tile_size(&self) -> UVec2 {
        UVec2::new(self.tile.width() as u32, self.tile.height() as u32)
    }
}
