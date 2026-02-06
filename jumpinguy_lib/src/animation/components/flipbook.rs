use std::collections::HashMap;

use bevy::ecs::component::Component;

use crate::animation::Animation;

#[derive(Component, Debug)]
pub struct Flipbook {
    animations: HashMap<String, Animation>,
    current: String,
}
