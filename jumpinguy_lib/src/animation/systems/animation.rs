use bevy::{
    ecs::system::{Query, Res},
    sprite::Sprite,
    time::Time,
};

use crate::animation::Animation;

pub(in crate::animation) fn tick_animated_sprites(
    mut animated_sprites: Query<(&mut Animation, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in &mut animated_sprites {
        // Tick the animation forward by the elapsed time
        let (previous_frame, frame) = animation.tick(time.delta());

        // If the animation advanced, increment the sprite's texture atlas
        if previous_frame != frame
            && let Some(texture_atlas) = &mut sprite.texture_atlas
        {
            texture_atlas.index = frame;
        }
    }
}
