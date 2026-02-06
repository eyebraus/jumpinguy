use bevy::{
    app::{App, Plugin},
    asset::AssetApp,
};

use crate::sheet::assets::{SheetAsset, SheetAssetLoader};

pub(crate) struct SheetPlugin;

impl Plugin for SheetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SheetAsset>()
            .register_asset_loader(SheetAssetLoader::default());
    }
}
