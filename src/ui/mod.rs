use std::time::Duration;

use bevy::prelude::*;
use bevy_gltf_blueprints::{AnimationPlayerLink, Animations};

/// Registers types and systems for UI tag components
#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Spinner>()
            .add_systems(Update, play_spin_animation);
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct Spinner;

fn play_spin_animation(
    spinners: Query<(&AnimationPlayerLink, &Animations), Added<Spinner>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for (link, animations) in spinners.iter() {
        let Ok((mut animation_player, mut animation_transitions)) =
            animation_players.get_mut(link.0)
        else {
            error!("No animation player found");
            continue;
        };

        let Some(&new_animation) = animations.named_indices.get("spin") else {
            error!("Spinner has no `spin` animation");
            continue;
        };
        animation_transitions
            .play(&mut animation_player, new_animation, Duration::ZERO)
            .repeat();
    }
}
