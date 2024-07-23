use bevy::prelude::*;

use crate::states::AppState;

mod panel;
mod spinner;

/// Registers types and systems for UI tag components
#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spinner::setup_spinners, spinner::spin_spinners)
                .run_if(not(in_state(AppState::CoreLoading))),
        )
        .register_type::<spinner::Spinner>()
        .register_type::<panel::Panel>();
    }
}
