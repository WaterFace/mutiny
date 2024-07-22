use bevy::{log::LogPlugin, prelude::*};
use bevy_gltf_blueprints::{BlueprintsPlugin, GltfFormat};
use bevy_registry_export::ExportRegistryPlugin;

mod assets;
mod main_menu;
mod states;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        }))
        .add_plugins(ExportRegistryPlugin::default())
        .add_plugins(BlueprintsPlugin {
            legacy_mode: false,
            library_folder: "models/library".into(),
            material_library: true,
            material_library_folder: "materials".into(),
            format: GltfFormat::GLB,
            aabbs: true,
            ..Default::default()
        })
        .add_plugins(states::StatesPlugin)
        .add_plugins(assets::AssetsPlugin::default())
        .add_plugins(ui::UiPlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .run();
}
