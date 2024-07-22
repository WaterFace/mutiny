use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[allow(unused)]
#[derive(AssetCollection, Resource)]
pub struct CoreAssets {
    /// Global game font.
    #[asset(key = "font")]
    pub font: Handle<Font>,

    /// spinner texture. represents loading, saving, reloading, etc.
    #[asset(key = "spinner_texture")]
    pub spinner_texture: Handle<Image>,

    /// panel texture. used for non-button ui panels. should be a ninepatch texture
    #[asset(key = "panel_texture")]
    pub panel_texture: Handle<Image>,

    /// button texture. used for ui buttons. should be a ninepatch texture, and a three-by-one spritesheet
    #[asset(key = "button_texture")]
    pub button_texture: Handle<Image>,
}
