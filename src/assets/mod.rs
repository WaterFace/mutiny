use std::borrow::Cow;

use bevy::prelude::*;

use bevy_asset_loader::prelude::*;

mod assets_core;
pub use assets_core::*;

mod assets_game;
pub use assets_game::*;

use crate::states::AppState;

pub struct AssetsPlugin {
    pub core_assets_file: Cow<'static, str>,
    pub game_assets_file: Cow<'static, str>,
}

impl Default for AssetsPlugin {
    fn default() -> Self {
        AssetsPlugin {
            core_assets_file: "assets_core.assets.ron".into(),
            game_assets_file: "assets_game.assets.ron".into(),
        }
    }
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::CoreLoading)
                .continue_to_state(AppState::InMenu)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(&self.core_assets_file)
                .load_collection::<CoreAssets>(),
        )
        .add_loading_state(
            LoadingState::new(AppState::AppLoading)
                .continue_to_state(AppState::AppRunning)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(&self.game_assets_file)
                .load_collection::<GameAssets>(),
        );
    }
}
