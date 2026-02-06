use bevy::{asset::Handle, ecs::component::Component, image::Image};

use crate::sheet::SheetAsset;

#[derive(Clone, Component, Debug)]
pub(crate) struct Sheet {
    configuration: Handle<SheetAsset>,
    image: Handle<Image>,
}

impl Sheet {
    pub fn new(image: Handle<Image>, configuration: Handle<SheetAsset>) -> Self {
        Self {
            configuration,
            image,
        }
    }

    pub fn configuration(&self) -> &Handle<SheetAsset> {
        &self.configuration
    }

    pub fn image(&self) -> &Handle<Image> {
        &self.image
    }
}
