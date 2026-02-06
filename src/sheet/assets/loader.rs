use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    reflect::TypePath,
};
use serde_yml::from_slice;
use thiserror::Error;

use crate::sheet::assets::sheet::SheetAsset;

const EXTENSIONS: [&'static str; 1] = ["sheet.yaml"];

#[derive(Default, TypePath)]
pub(in crate::sheet) struct SheetAssetLoader;

impl AssetLoader for SheetAssetLoader {
    type Asset = SheetAsset;
    type Error = SheetAssetLoaderError;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &EXTENSIONS
    }

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let sheet = from_slice::<Self::Asset>(&bytes)?;

        Ok(sheet)
    }
}

#[derive(Debug, Error)]
pub(crate) enum SheetAssetLoaderError {
    #[error(transparent)]
    DeserializationError(#[from] serde_yml::Error),

    #[error(transparent)]
    ReaderError(#[from] std::io::Error),
}
