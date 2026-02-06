use bevy::app::{App, Plugin, Update};

use crate::animation::systems::animation::tick_animated_sprites;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_animated_sprites);
    }
}
