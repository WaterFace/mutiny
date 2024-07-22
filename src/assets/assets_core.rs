use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

#[allow(unused)]
#[derive(AssetCollection, Resource)]
pub struct CoreAssets {
    /// Global game font.
    #[asset(key = "font", optional)]
    pub font: Option<Handle<Font>>,

    /// Main Menu scene
    #[asset(key = "main_menu_scene", optional)]
    pub main_menu_scene: Option<Handle<Gltf>>,

    /// GLTF models
    #[asset(key = "models", collection(typed, mapped), optional)]
    pub models: Option<HashMap<String, Handle<Gltf>>>,

    #[asset(key = "spinner", optional)]
    pub spinner: Option<Handle<Gltf>>,
}
